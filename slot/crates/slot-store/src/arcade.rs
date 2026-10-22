//! Names and BIOS filtering from the exact bundled FBNeo driver DAT.
const DRIVERS: &str=include_str!("../assets/fbneo-arcade.tsv");
pub fn entry(stem: &str) -> Option<(bool, &'static str)> {
 DRIVERS.lines().filter_map(|l| {
  let mut fields=l.splitn(3,'\t');
  let name=fields.next()?; let kind=fields.next()?; let title=fields.next()?;
  name.eq_ignore_ascii_case(stem).then_some((kind=="bios",title))
 }).next()
}
