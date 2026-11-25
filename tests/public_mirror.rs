//! Decode rows of every table on the public UDD mirror.
//!
//! These tests need network access and are ignored by default; run them with
//! `cargo test -- --ignored`.

use udd::*;

async fn check<T>(table: &str, sql: &'static str)
where
    T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    let pool = connect_public_mirror().await.unwrap();
    let rows: Vec<T> = sqlx::query_as(sql)
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|e| panic!("{table}: {e}"));
    let _ = rows;
}

macro_rules! table_tests {
    ($($name:ident: $ty:ty = $table:literal;)*) => {
        $(
            #[tokio::test]
            #[ignore = "needs network access"]
            async fn $name() {
                check::<$ty>($table, concat!("SELECT * FROM ", $table, " LIMIT 100")).await;
            }
        )*
    };
}

table_tests! {
    sources: Source = "sources";
    packages: Package = "packages";
    uploaders: Uploader = "uploaders";
    descriptions: Description = "descriptions";
    description_imports: DescriptionImport = "description_imports";
    packages_summary: PackageSummary = "packages_summary";
    packages_distrelcomparch: PackageDistRelCompArch = "packages_distrelcomparch";
    sources_files: SourceFile = "sources_files";
    releases: Release = "releases";
    timestamps: Timestamp = "timestamps";
    udd_logs: UddLog = "udd_logs";
    bugs: Bug = "bugs";
    bugs_blockedby: BugBlockedBy = "bugs_blockedby";
    bugs_blocks: BugBlocks = "bugs_blocks";
    bugs_fixed_in: BugFixedIn = "bugs_fixed_in";
    bugs_found_in: BugFoundIn = "bugs_found_in";
    bugs_merged_with: BugMergedWith = "bugs_merged_with";
    bugs_packages: BugPackage = "bugs_packages";
    bugs_stamps: BugStamps = "bugs_stamps";
    bugs_tags: BugTag = "bugs_tags";
    bugs_usertags: BugUsertag = "bugs_usertags";
    bts_tags: BtsTag = "bts_tags";
    pseudo_packages: PseudoPackage = "pseudo_packages";
    potential_bug_closures: PotentialBugClosure = "potential_bug_closures";
    upload_history: UploadHistory = "upload_history";
    upload_history_architecture: UploadHistoryArchitecture = "upload_history_architecture";
    upload_history_closes: UploadHistoryCloses = "upload_history_closes";
    deferred: Deferred = "deferred";
    deferred_architecture: DeferredArchitecture = "deferred_architecture";
    deferred_binary: DeferredBinary = "deferred_binary";
    deferred_closes: DeferredCloses = "deferred_closes";
    new_packages: NewPackage = "new_packages";
    new_sources: NewSource = "new_sources";
    mentors_raw_uploads: MentorsRawUpload = "mentors_raw_uploads";
    popcon: Popcon = "popcon";
    popcon_src: PopconSrc = "popcon_src";
    popcon_src_average: PopconSrcAverage = "popcon_src_average";
    carnivore_emails: CarnivoreEmail = "carnivore_emails";
    carnivore_keys: CarnivoreKey = "carnivore_keys";
    carnivore_login: CarnivoreLogin = "carnivore_login";
    carnivore_names: CarnivoreName = "carnivore_names";
    debian_maintainers: DebianMaintainer = "debian_maintainers";
    lintian_logs: LintianLog = "lintian_logs";
    lintian_results: LintianResult = "lintian_results";
    lintian_tags_descriptions: LintianTagDescription = "lintian_tags_descriptions";
    ftp_autorejects: FtpAutoreject = "ftp_autorejects";
    migrations: Migration = "migrations";
    migration_excuses: MigrationExcuse = "migration_excuses";
    hints: Hint = "hints";
    testing_autoremovals: TestingAutoremoval = "testing_autoremovals";
    key_packages: KeyPackage = "key_packages";
    package_removal: PackageRemoval = "package_removal";
    package_removal_batch: PackageRemovalBatch = "package_removal_batch";
    orphaned_packages: OrphanedPackage = "orphaned_packages";
    wannabuild: Wannabuild = "wannabuild";
    ci: Ci = "ci";
    piuparts_status: PiupartsStatus = "piuparts_status";
    reproduce: Reproduce = "reproduce";
    reproducible: Reproducible = "reproducible";
    reproducible_issues: ReproducibleIssue = "reproducible_issues";
    reproducible_notes: ReproducibleNote = "reproducible_notes";
    dehs: Dehs = "dehs";
    upstream: Upstream = "upstream";
    upstream_metadata: UpstreamMetadata = "upstream_metadata";
    vcs: Vcs = "vcs";
    vcswatch: Vcswatch = "vcswatch";
    salsa_projects: SalsaProject = "salsa_projects";
    sources_salsa_projects: SourceSalsaProject = "sources_salsa_projects";
    duck_raw: DuckRaw = "duck_raw";
    duck_url_status: DuckUrlStatus = "duck_url_status";
    duck_urls: DuckUrl = "duck_urls";
    debaudit: Debaudit = "debaudit";
    patches: Patch = "patches";
    patches_status: PatchStatus = "patches_status";
    multiarch_hints: MultiarchHint = "multiarch_hints";
    security_issues: SecurityIssue = "security_issues";
    security_issues_releases: SecurityIssueRelease = "security_issues_releases";
    bibref: Bibref = "bibref";
    blends_dependencies: BlendDependency = "blends_dependencies";
    blends_dependencies_alternatives: BlendDependencyAlternative = "blends_dependencies_alternatives";
    blends_dependencies_priorities: BlendDependencyPriority = "blends_dependencies_priorities";
    blends_metadata: BlendMetadata = "blends_metadata";
    blends_prospectivepackages: BlendProspectivePackage = "blends_prospectivepackages";
    blends_remarks: BlendRemark = "blends_remarks";
    blends_tasks: BlendTask = "blends_tasks";
    blends_unknown_packages: BlendUnknownPackage = "blends_unknown_packages";
    edam: Edam = "edam";
    registry: Registry = "registry";
    screenshots: Screenshot = "screenshots";
    debtags: Debtag = "debtags";
    ubuntu_bugs: UbuntuBug = "ubuntu_bugs";
    ubuntu_bugs_duplicates: UbuntuBugDuplicate = "ubuntu_bugs_duplicates";
    ubuntu_bugs_subscribers: UbuntuBugSubscriber = "ubuntu_bugs_subscribers";
    ubuntu_bugs_tags: UbuntuBugTag = "ubuntu_bugs_tags";
    ubuntu_bugs_tasks: UbuntuBugTask = "ubuntu_bugs_tasks";
    ubuntu_upload_history: UbuntuUploadHistory = "ubuntu_upload_history";
    ubuntu_upload_history_launchpad_closes: UbuntuUploadHistoryLaunchpadCloses = "ubuntu_upload_history_launchpad_closes";
}

#[tokio::test]
#[ignore = "needs network access"]
async fn ci_non_version() {
    check::<Ci>("ci", "SELECT * FROM ci WHERE version = 'n/a' LIMIT 10").await;
}

#[tokio::test]
#[ignore = "needs network access"]
async fn hints_version_with_arch() {
    check::<Hint>(
        "hints",
        "SELECT * FROM hints WHERE version::text LIKE '%/%' LIMIT 10",
    )
    .await;
}

#[tokio::test]
#[ignore = "needs network access"]
async fn source_version() {
    let pool = connect_public_mirror().await.unwrap();
    let source: Source = sqlx::query_as(
        "SELECT * FROM sources WHERE source = 'dpkg' AND release = 'sid' AND component = 'main'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(source.source, "dpkg");
    assert!(source.version > "1.20".parse::<Version>().unwrap());
}

#[tokio::test]
#[ignore = "needs network access"]
async fn bug_severity() {
    let pool = connect_public_mirror().await.unwrap();
    let severity: BugSeverity = sqlx::query_scalar("SELECT 'serious'::bugs_severity")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(severity, BugSeverity::Serious);
    assert!(severity.is_release_critical());
}
