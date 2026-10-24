//! Token savings analytics and cost reporting.

#[cfg(feature = "tracking")]
pub mod cc_economics;
#[cfg(feature = "tracking")]
pub mod ccusage;
#[cfg(feature = "tracking")]
pub mod gain;

/// Stats commands need the SQLite history; builds without `tracking` keep the CLI surface but refuse.
#[cfg(not(feature = "tracking"))]
pub mod gain {
    #[allow(clippy::too_many_arguments)]
    pub fn run(
        _: bool, _: bool, _: bool, _: bool, _: &str, _: bool, _: bool, _: bool, _: bool, _: &str,
        _: bool, _: bool, _: bool, _: u8,
    ) -> anyhow::Result<()> {
        anyhow::bail!("token tracking is not compiled into this build")
    }
}

#[cfg(not(feature = "tracking"))]
pub mod cc_economics {
    pub fn run(_: bool, _: bool, _: bool, _: bool, _: &str, _: u8) -> anyhow::Result<()> {
        anyhow::bail!("token tracking is not compiled into this build")
    }
}
pub mod session_cmd;
