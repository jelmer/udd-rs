"""Generate Rust row structs for UDD tables from the public mirror schema."""

import os
import sys
from collections import defaultdict

import psycopg2

OUT = sys.argv[1]

# table -> (module, struct name)
TABLES = {
    "sources": ("archive", "Source"),
    "packages": ("archive", "Package"),
    "uploaders": ("archive", "Uploader"),
    "descriptions": ("archive", "Description"),
    "description_imports": ("archive", "DescriptionImport"),
    "packages_summary": ("archive", "PackageSummary"),
    "packages_distrelcomparch": ("archive", "PackageDistRelCompArch"),
    "sources_files": ("archive", "SourceFile"),
    "releases": ("archive", "Release"),
    "timestamps": ("archive", "Timestamp"),
    "udd_logs": ("archive", "UddLog"),
    "bugs": ("bugs", "Bug"),
    "bugs_blockedby": ("bugs", "BugBlockedBy"),
    "bugs_blocks": ("bugs", "BugBlocks"),
    "bugs_fixed_in": ("bugs", "BugFixedIn"),
    "bugs_found_in": ("bugs", "BugFoundIn"),
    "bugs_merged_with": ("bugs", "BugMergedWith"),
    "bugs_packages": ("bugs", "BugPackage"),
    "bugs_stamps": ("bugs", "BugStamps"),
    "bugs_tags": ("bugs", "BugTag"),
    "bugs_usertags": ("bugs", "BugUsertag"),
    "bts_tags": ("bugs", "BtsTag"),
    "pseudo_packages": ("bugs", "PseudoPackage"),
    "potential_bug_closures": ("bugs", "PotentialBugClosure"),
    "upload_history": ("upload_history", "UploadHistory"),
    "upload_history_architecture": ("upload_history", "UploadHistoryArchitecture"),
    "upload_history_closes": ("upload_history", "UploadHistoryCloses"),
    "deferred": ("upload_history", "Deferred"),
    "deferred_architecture": ("upload_history", "DeferredArchitecture"),
    "deferred_binary": ("upload_history", "DeferredBinary"),
    "deferred_closes": ("upload_history", "DeferredCloses"),
    "new_packages": ("upload_history", "NewPackage"),
    "new_sources": ("upload_history", "NewSource"),
    "mentors_raw_uploads": ("upload_history", "MentorsRawUpload"),
    "popcon": ("popcon", "Popcon"),
    "popcon_src": ("popcon", "PopconSrc"),
    "popcon_src_average": ("popcon", "PopconSrcAverage"),
    "carnivore_emails": ("carnivore", "CarnivoreEmail"),
    "carnivore_keys": ("carnivore", "CarnivoreKey"),
    "carnivore_login": ("carnivore", "CarnivoreLogin"),
    "carnivore_names": ("carnivore", "CarnivoreName"),
    "debian_maintainers": ("carnivore", "DebianMaintainer"),
    "lintian_logs": ("lintian", "LintianLog"),
    "lintian_results": ("lintian", "LintianResult"),
    "lintian_tags_descriptions": ("lintian", "LintianTagDescription"),
    "ftp_autorejects": ("lintian", "FtpAutoreject"),
    "migrations": ("testing", "Migration"),
    "migration_excuses": ("testing", "MigrationExcuse"),
    "hints": ("testing", "Hint"),
    "testing_autoremovals": ("testing", "TestingAutoremoval"),
    "key_packages": ("testing", "KeyPackage"),
    "package_removal": ("testing", "PackageRemoval"),
    "package_removal_batch": ("testing", "PackageRemovalBatch"),
    "orphaned_packages": ("testing", "OrphanedPackage"),
    "wannabuild": ("testing", "Wannabuild"),
    "ci": ("qa", "Ci"),
    "piuparts_status": ("qa", "PiupartsStatus"),
    "reproduce": ("qa", "Reproduce"),
    "reproducible": ("qa", "Reproducible"),
    "reproducible_issues": ("qa", "ReproducibleIssue"),
    "reproducible_notes": ("qa", "ReproducibleNote"),
    "dehs": ("qa", "Dehs"),
    "upstream": ("qa", "Upstream"),
    "upstream_metadata": ("qa", "UpstreamMetadata"),
    "vcs": ("qa", "Vcs"),
    "vcswatch": ("qa", "Vcswatch"),
    "salsa_projects": ("qa", "SalsaProject"),
    "sources_salsa_projects": ("qa", "SourceSalsaProject"),
    "duck_raw": ("qa", "DuckRaw"),
    "duck_url_status": ("qa", "DuckUrlStatus"),
    "duck_urls": ("qa", "DuckUrl"),
    "debaudit": ("qa", "Debaudit"),
    "patches": ("qa", "Patch"),
    "patches_status": ("qa", "PatchStatus"),
    "multiarch_hints": ("qa", "MultiarchHint"),
    "security_issues": ("security", "SecurityIssue"),
    "security_issues_releases": ("security", "SecurityIssueRelease"),
    "bibref": ("blends", "Bibref"),
    "blends_dependencies": ("blends", "BlendDependency"),
    "blends_dependencies_alternatives": ("blends", "BlendDependencyAlternative"),
    "blends_dependencies_priorities": ("blends", "BlendDependencyPriority"),
    "blends_metadata": ("blends", "BlendMetadata"),
    "blends_prospectivepackages": ("blends", "BlendProspectivePackage"),
    "blends_remarks": ("blends", "BlendRemark"),
    "blends_tasks": ("blends", "BlendTask"),
    "blends_unknown_packages": ("blends", "BlendUnknownPackage"),
    "edam": ("blends", "Edam"),
    "registry": ("blends", "Registry"),
    "screenshots": ("blends", "Screenshot"),
    "debtags": ("blends", "Debtag"),
    "ubuntu_bugs": ("ubuntu", "UbuntuBug"),
    "ubuntu_bugs_duplicates": ("ubuntu", "UbuntuBugDuplicate"),
    "ubuntu_bugs_subscribers": ("ubuntu", "UbuntuBugSubscriber"),
    "ubuntu_bugs_tags": ("ubuntu", "UbuntuBugTag"),
    "ubuntu_bugs_tasks": ("ubuntu", "UbuntuBugTask"),
    "ubuntu_upload_history": ("ubuntu", "UbuntuUploadHistory"),
    "ubuntu_upload_history_launchpad_closes": (
        "ubuntu",
        "UbuntuUploadHistoryLaunchpadCloses",
    ),
}

# Tables that share the schema of another table; verified below.
ALIASES = {
    "migration_excuses_experimental": "migration_excuses",
}
for base in [
    "descriptions",
    "packages",
    "packages_distrelcomparch",
    "packages_summary",
    "sources",
    "uploaders",
]:
    for prefix in ["archived", "derivatives", "ports", "ubuntu", "unofficial"]:
        ALIASES[f"{prefix}_{base}"] = base
for t in [
    "bugs",
    "bugs_blockedby",
    "bugs_blocks",
    "bugs_fixed_in",
    "bugs_found_in",
    "bugs_merged_with",
    "bugs_packages",
    "bugs_stamps",
    "bugs_tags",
]:
    ALIASES[f"archived_{t}"] = t
for t in [
    "description_imports",
    "popcon",
    "popcon_src",
    "popcon_src_average",
    "upload_history_closes",
]:
    ALIASES[f"ubuntu_{t}"] = t

ENUMS = {
    "bugs_severity": "BugSeverity",
    "dehs_status": "DehsStatus",
    "ftp_autoreject_level": "FtpAutorejectLevel",
    "ftp_autoreject_type": "FtpAutorejectType",
    "lintian_tagtype": "LintianTagType",
    "security_issues_releases_status": "SecurityIssueReleaseStatus",
    "security_issues_scope": "SecurityIssueScope",
}

TYPES = {
    "text": "String",
    "bpchar": "String",
    "int4": "i32",
    "int8": "i64",
    "bool": "bool",
    "timestamp": "NaiveDateTime",
    "timestamptz": "DateTime<Utc>",
    "date": "NaiveDate",
    "jsonb": "JsonValue",
    "_text": "Vec<String>",
    "_int4": "Vec<i32>",
    "numeric": "Decimal",
    "interval": "PgInterval",
    "debversion": "Version",
}
TYPES.update(ENUMS)

# debversion columns that hold values which are not valid Debian versions.
OVERRIDES = {
    ("ci", "version"): "CiVersion",
    ("hints", "version"): "HintVersion",
}

KEYWORDS = {
    "type",
    "override",
    "match",
    "ref",
    "move",
    "use",
    "mod",
    "fn",
    "impl",
    "struct",
    "enum",
    "loop",
    "where",
    "in",
    "as",
    "self",
}

conn = psycopg2.connect(
    host="udd-mirror.debian.net", user="udd-mirror", password="udd-mirror", dbname="udd"
)
cur = conn.cursor()
cur.execute("""
select c.table_name, c.column_name, c.udt_name, c.is_nullable
from information_schema.columns c
join information_schema.tables t using (table_schema, table_name)
where c.table_schema = 'public' and t.table_type = 'BASE TABLE'
order by c.table_name, c.ordinal_position""")
columns = defaultdict(list)
for table, col, udt, nullable in cur.fetchall():
    columns[table].append((col, udt, nullable == "YES"))

for alias, base in ALIASES.items():
    if columns[alias] != columns[base]:
        raise SystemExit(f"{alias} does not match {base}")

aliases_of = defaultdict(list)
for alias, base in ALIASES.items():
    aliases_of[base].append(alias)

modules = defaultdict(list)
for table, (module, name) in TABLES.items():
    if table not in columns:
        raise SystemExit(f"missing table {table}")
    modules[module].append((table, name))

HEADER = "// Generated by scripts/generate-tables.py from the UDD schema.\n"

for module, tables in modules.items():
    used = set()
    body = []
    for table, name in tables:
        doc = [f"/// A row in the `{table}` table."]
        if aliases_of[table]:
            doc.append("///")
            doc.append(
                "/// Also matches the rows of "
                + ", ".join(f"`{a}`" for a in sorted(aliases_of[table]))
                + "."
            )
        lines = doc + [
            "#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]",
            '#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]',
            f"pub struct {name} {{",
        ]
        for col, udt, nullable in columns[table]:
            rtype = TYPES[udt]
            rtype = OVERRIDES.get((table, col), rtype)
            used.add(rtype)
            if nullable:
                rtype = f"Option<{rtype}>"
            field = col
            if col in KEYWORDS:
                field = f"{col}_"
                lines.append(f'    #[sqlx(rename = "{col}")]')
                lines.append(
                    f'    #[cfg_attr(feature = "serde", serde(rename = "{col}"))]'
                )
            if udt == "interval":
                # PgInterval does not implement serde traits.
                lines.append('    #[cfg_attr(feature = "serde", serde(skip))]')
            lines.append(f"    pub {field}: {rtype},")
        lines.append("}")
        body.append("\n".join(lines))

    imports = []
    chrono = sorted(t for t in ["NaiveDateTime", "NaiveDate"] if t in used)
    if "DateTime<Utc>" in used:
        chrono += ["DateTime", "Utc"]
    if chrono:
        imports.append(
            f"use chrono::{{{', '.join(sorted(chrono))}}};"
            if len(chrono) > 1
            else f"use chrono::{chrono[0]};"
        )
    if "Version" in used:
        imports.append("use debversion::Version;")
    if "Decimal" in used:
        imports.append("use rust_decimal::Decimal;")
    if "JsonValue" in used:
        imports.append("use serde_json::Value as JsonValue;")
    if "PgInterval" in used:
        imports.append("use sqlx::postgres::types::PgInterval;")
    for t in ["CiVersion", "HintVersion"]:
        if t in used:
            imports.append(f"use crate::types::{t};")
    enums = sorted(e for e in ENUMS.values() if e in used)
    if enums:
        imports.append(
            f"use crate::enums::{{{', '.join(enums)}}};"
            if len(enums) > 1
            else f"use crate::enums::{enums[0]};"
        )

    with open(os.path.join(OUT, f"{module}.rs"), "w") as f:
        f.write(HEADER)
        if imports:
            f.write("\n" + "\n".join(imports) + "\n")
        f.write("\n" + "\n\n".join(body) + "\n")

print(" ".join(modules))
