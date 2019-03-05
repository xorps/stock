use crate::core::{PackageIdSpec, Workspace};
use crate::ops;
use crate::util::CargoResult;
use crate::ops::lockfile::LOCKFILE_FILENAME;

pub fn pkgid(ws: &Workspace<'_>, spec: Option<&str>) -> CargoResult<PackageIdSpec> {
    let resolve = match ops::load_pkg_lockfile(ws)? {
        Some(resolve) => resolve,
        None => failure::bail!("a {} must exist for this command", LOCKFILE_FILENAME),
    };

    let pkgid = match spec {
        Some(spec) => PackageIdSpec::query_str(spec, resolve.iter())?,
        None => ws.current()?.package_id(),
    };
    Ok(PackageIdSpec::from_package_id(pkgid))
}
