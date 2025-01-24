pub fn try_fips_mode() -> Result<(), &'static str> {
    aws_lc_rs::try_fips_mode()
}