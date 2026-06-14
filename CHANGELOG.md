## v0.2.5 (Jun 14, 2026)

- Tier 1 (9 patterns): Piercing Line, Three Outside Up/Down, Tweezer Top/Bottom, Bullish/Bearish Harami Cross, Bullish/Bearish Kicker
- Tier 2 (12 patterns): Bullish/Bearish Abandoned Baby, Bullish/Bearish Tri-Star, Bullish/Bearish Three Line Strike, Advance Block, Deliberation, Bullish/Bearish Belt Hold, High Wave, Four-Price Doji
- Adds three new overridable trait methods on `CandleStick`: `tweezer_tolerance` (default 5%), `belt_hold_body_ratio` (default 70%), `high_wave_shadow_ratio` (default 40%)

## v0.2.4 (May 6, 2026)

- Removed lifetime parameter  - contribution by @joelchen (#5)
- Added is_downtrend, is_uptrend functions  - contribution by @joelchen (#5)

## v0.2.3 (Dec 22, 2025)

- Implemented CandleStick trait for tuple reference

## v0.2.2 (Dec 6, 2025)

- Bug fixed in get and prev functions  - contribution by @flarocca (#2)

## v0.2.1 (Dec 5, 2025)

- Added Three Inside Up/Down patterns - contribution by @flarocca (#1)

## v0.2.0 (May 15, 2025)

- Added volume to the CandleStick trait
- CandleStick trait object safe
- Added typical price and money flow

## v0.1.1 (May 08, 2025)

- Fixed docs.rs url for documentation 

## v0.1.0 (May 07, 2025)

- Initial release.
