use crate::{utils::midpoint, CandleStick};

const SERIES_SIZE: usize = 5;

/// The `CandleStream` provides detection capabilities for powerful multi-candle patterns
///
/// - **Reversal Patterns**: Engulfing, Harami, Morning/Evening Stars, Doji Stars
/// - **Continuation Patterns**: Three White Soldiers, Three Black Crows
/// - **Top/Bottom Formations**: Dark Cloud Cover and other significant reversal signals
///
/// These formations often provide stronger trading signals than single-candle patterns,
/// offering insights into potential trend reversals, continuations, or exhaustion points.
/// Each pattern detection method includes detailed documentation about market context
/// and trading significance.
///
/// # Examples
///
/// ```
/// use candlestick_rs::{CandleStick, CandleStream};
///
/// // Create a new stream and add candles
/// let candle1 = (100.0, 105.0, 99.0, 104.0, 0.0);
/// let candle2 = (104.5, 110.0, 104.0, 109.0, 0.0);
///
/// let mut stream = CandleStream::new();
/// stream.push(&candle1).push(&candle2);
///
/// // Check for patterns
/// if stream.is_bullish_engulfing() {
///     println!("Bullish engulfing pattern detected!");
/// }
/// ```

#[derive(Debug)]
pub struct CandleStream<T> {
    series: [Option<T>; SERIES_SIZE],
    idx: usize,
}

impl<T> CandleStream<T> {
    /// Returns a new candle series
    pub fn new() -> Self {
        Self::default()
    }

    // Returns the index of the nth last candle
    fn nth_index(&self, n: usize) -> Option<usize> {
        if n > SERIES_SIZE {
            return None;
        }

        Some((self.idx + SERIES_SIZE - n) % SERIES_SIZE)
    }

    // Returns the candle at the given index
    fn at(&self, idx: usize) -> Option<&T> {
        match idx < SERIES_SIZE {
            true => self.series[idx].as_ref(),
            false => None,
        }
    }

    // Fetches reference to the current candle
    fn get(&self) -> Option<&T> {
        self.at(self.nth_index(1)?)
    }

    // Returns the previous candle
    fn prev(&self, n: usize) -> Option<&T> {
        self.at(self.nth_index(n + 1)?)
    }

    /// Pushes a candle to the series
    pub fn push(&mut self, candle: T) -> &mut Self {
        self.series[self.idx % SERIES_SIZE] = Some(candle);
        self.idx = (self.idx + 1) % SERIES_SIZE;
        self
    }
}

impl<T: CandleStick> CandleStream<T> {
    /// Identifies a Bullish Doji Star pattern, a potential reversal signal in downtrends.
    ///
    /// This two-candle pattern occurs when a bearish candle is followed by a Doji that gaps below
    /// the prior candle's low. The Doji represents market indecision after a dominant downtrend.
    ///
    /// **Trading Significance**:
    /// - Signals potential exhaustion of selling pressure
    /// - Often precedes bullish price movements when confirmed
    /// - Traders typically wait for a third bullish candle before entering long positions
    /// - Most effective when appearing at support levels or after extended downtrends
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (52.0, 52.5, 48.0, 48.5, 0.0);
    /// let curr = (47.0, 47.5, 46.8, 47.0, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_bullish_doji_star());
    /// ```
    pub fn is_bullish_doji_star(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .is_some_and(|(c, p)| p.is_bearish() && c.is_doji() && c.high() < p.low())
    }

    /// Identifies a Bearish Doji Star pattern, a potential reversal signal in uptrends.
    ///
    /// This two-candle pattern occurs when a bullish candle is followed by a Doji that gaps above
    /// the prior candle's high. The Doji represents market indecision after a dominant uptrend.
    ///
    /// **Trading Significance**:
    /// - Signals potential exhaustion of buying pressure
    /// - Often precedes bearish price movements when confirmed
    /// - Traders typically wait for a third bearish candle before entering short positions
    /// - Most effective when appearing at resistance levels or after extended uptrends
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (48.0, 52.5, 47.8, 52.0, 0.0);
    /// let curr = (52.6, 53.2, 52.6, 52.6, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_bearish_doji_star());
    /// ```
    pub fn is_bearish_doji_star(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .is_some_and(|(c, p)| p.is_bullish() && c.is_doji() && c.low() > p.high())
    }

    ///
    /// Identifies a Bullish Engulfing pattern, a strong reversal signal at the end of downtrends.
    ///
    /// This two-candle pattern occurs when a bearish candle is completely engulfed by a larger bullish candle
    /// (open lower than prior close, close higher than prior open). It shows buyers overwhelmingly defeating sellers.
    ///
    /// **Trading Significance**:
    /// - Indicates strong shift from selling to buying pressure
    /// - More reliable than single-candle patterns due to the decisive price action
    /// - Often used as an immediate entry signal, especially when volume increases
    /// - Higher reliability when occurring at support zones or after extended downtrends
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (101.0, 102.0, 99.5, 100.5, 0.0); // bearish: open > close
    /// let curr = (99.0, 103.0, 98.5, 102.5, 0.0);  // bullish: open < close, engulfs prev body
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_bullish_engulfing());
    /// ```
    pub fn is_bullish_engulfing(&self) -> bool {
        self.get().zip(self.prev(1)).is_some_and(|(c, p)| {
            p.is_bearish() && c.is_bullish() && c.open() < p.close() && c.close() > p.open()
        })
    }

    /// Identifies a Bearish Engulfing pattern, a strong reversal signal at the end of uptrends.
    ///
    /// This two-candle pattern occurs when a bullish candle is completely engulfed by a larger bearish candle
    /// (open higher than prior close, close lower than prior open). It shows sellers overwhelmingly defeating buyers.
    ///
    /// **Trading Significance**:
    /// - Indicates strong shift from buying to selling pressure
    /// - More reliable than single-candle patterns due to the decisive price action
    /// - Often used as an immediate exit signal for longs or entry for shorts
    /// - Higher reliability when occurring at resistance zones or after extended uptrends
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (99.0, 100.5, 98.5, 100.0, 0.0);  // bullish: open < close
    /// let curr = (101.5, 102.0, 97.0, 98.5, 0.0);  // bearish: open > close, engulfs prev body
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_bearish_engulfing());
    /// ```
    pub fn is_bearish_engulfing(&self) -> bool {
        self.get().zip(self.prev(1)).is_some_and(|(c, p)| {
            p.is_bullish() && c.is_bearish() && c.open() > p.close() && c.close() < p.open()
        })
    }

    /// Identifies a Bullish Kicker pattern, one of the strongest two-candle reversal signals.
    ///
    /// This pattern consists of a bearish marubozu followed by a bullish marubozu
    /// that opens **at or above** the prior open — leaving no overlap between the
    /// two bodies. The trade sentiment flips violently between sessions with no
    /// continuation of the prior bearishness.
    ///
    /// **Trading Significance**:
    /// - Among the highest-conviction reversal signals on the chart
    /// - Typically driven by news / earnings / macro events between sessions
    /// - The gap-up open with no shadow speaks to a complete shift in market
    ///   regime — late-trend shorts are immediately offsides
    /// - Often used as an immediate entry signal without further confirmation
    /// - More reliable when the prior trend was extended and the marubozu
    ///   bodies are large
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (100.0, 100.0, 95.0, 95.0, 0.0); // bearish marubozu
    /// let curr = (105.0, 110.0, 105.0, 110.0, 0.0); // bullish marubozu, opens >= prev.open
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_bullish_kicker());
    /// ```
    pub fn is_bullish_kicker(&self) -> bool {
        self.get().zip(self.prev(1)).is_some_and(|(c, p)| {
            p.is_bearish_marubozu() && c.is_bullish_marubozu() && c.open() >= p.open()
        })
    }

    /// Identifies a Bearish Kicker pattern, the mirror of the Bullish Kicker.
    ///
    /// This pattern consists of a bullish marubozu followed by a bearish marubozu
    /// that opens **at or below** the prior open — leaving no overlap between the
    /// two bodies. The trade sentiment flips violently downward between sessions
    /// with no continuation of the prior bullishness.
    ///
    /// **Trading Significance**:
    /// - Among the highest-conviction bearish reversal signals on the chart
    /// - Typically driven by news / earnings / macro events between sessions
    /// - The gap-down open with no shadow speaks to a complete shift in market
    ///   regime — late-trend longs are immediately offsides
    /// - Often used as an immediate entry signal for shorts or exit signal for
    ///   longs without further confirmation
    /// - More reliable when the prior trend was extended and the marubozu bodies
    ///   are large
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (95.0, 100.0, 95.0, 100.0, 0.0); // bullish marubozu
    /// let curr = (90.0, 90.0, 85.0, 85.0, 0.0); // bearish marubozu, opens <= prev.open
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_bearish_kicker());
    /// ```
    pub fn is_bearish_kicker(&self) -> bool {
        self.get().zip(self.prev(1)).is_some_and(|(c, p)| {
            p.is_bullish_marubozu() && c.is_bearish_marubozu() && c.open() <= p.open()
        })
    }

    /// Identifies a Bullish Harami pattern, indicating potential reversal or continuation in downtrends.
    ///
    /// This two-candle pattern occurs when a small bullish candle is contained within the trading range of a
    /// preceding larger bearish candle. The Japanese word "harami" means pregnant, describing the visual appearance.
    ///
    /// **Trading Significance**:
    /// - Signals indecision after a bearish move and possible loss of downward momentum
    /// - Less powerful than engulfing patterns but still a notable reversal signal
    /// - Traders typically wait for additional confirmation before entering long positions
    /// - Part of contingent trading strategies where position size increases after confirmation
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (129.0, 130.0, 124.0, 125.0, 0.0);
    /// let curr = (125.2, 127.0, 124.8, 126.5, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_bullish_harami());
    /// ```
    pub fn is_bullish_harami(&self) -> bool {
        self.get().zip(self.prev(1)).is_some_and(|(c, p)| {
            p.is_bearish() && c.is_bullish() && c.open() > p.close() && c.close() < p.open()
        })
    }

    /// Identifies a Bearish Harami pattern, indicating potential reversal or continuation in uptrends.
    ///
    /// This two-candle pattern occurs when a small bearish candle is contained within the trading range of a
    /// preceding larger bullish candle. The Japanese word "harami" means pregnant, describing the visual appearance.
    ///
    /// **Trading Significance**:
    /// - Signals indecision after a bullish move and possible loss of upward momentum
    /// - Less powerful than engulfing patterns but still a notable reversal warning
    /// - Often used to protect profits on long positions or tighten stop losses
    /// - Sometimes precedes a period of consolidation rather than immediate reversal
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (124.0, 129.0, 122.0, 127.0, 0.0);
    /// let curr = (126.9, 129.7, 125.0, 124.8, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_bearish_harami());
    /// ```
    pub fn is_bearish_harami(&self) -> bool {
        self.get().zip(self.prev(1)).is_some_and(|(c, p)| {
            p.is_bullish() && c.is_bearish() && c.open() < p.close() && c.close() > p.open()
        })
    }

    /// Identifies a Bullish Harami Cross pattern, a stronger variant of the Bullish Harami.
    ///
    /// This two-candle pattern occurs when a bearish candle is followed by a Doji whose
    /// body is fully contained inside the prior body. The Doji's perfect indecision
    /// after a bearish session is a more potent reversal signal than the small bullish
    /// body of a standard harami.
    ///
    /// **Trading Significance**:
    /// - Stronger than a standard Bullish Harami due to the Doji's heightened indecision
    /// - The market opened inside the prior body and failed to make meaningful progress
    ///   in either direction — bearish momentum has stalled
    /// - Often used as an early-warning signal; traders typically wait for a third
    ///   bullish candle before entering long positions
    /// - Particularly effective at support levels or after extended downtrends
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (100.0, 100.5, 95.0, 96.0, 0.0); // bearish, body [96, 100]
    /// let curr = (98.0, 98.5, 97.5, 98.0, 0.0); // doji inside prior body
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_bullish_harami_cross());
    /// ```
    pub fn is_bullish_harami_cross(&self) -> bool {
        self.get().zip(self.prev(1)).is_some_and(|(c, p)| {
            let upper = c.open().max(c.close());
            let lower = c.open().min(c.close());
            p.is_bearish() && c.is_doji() && upper < p.open() && lower > p.close()
        })
    }

    /// Identifies a Bearish Harami Cross pattern, a stronger variant of the Bearish Harami.
    ///
    /// This two-candle pattern occurs when a bullish candle is followed by a Doji whose
    /// body is fully contained inside the prior body. The Doji's perfect indecision
    /// after a bullish session is a more potent reversal signal than the small bearish
    /// body of a standard harami.
    ///
    /// **Trading Significance**:
    /// - Stronger than a standard Bearish Harami due to the Doji's heightened indecision
    /// - The market opened inside the prior body and failed to make meaningful progress
    ///   in either direction — bullish momentum has stalled
    /// - Often used as an early-warning signal; traders typically wait for a third
    ///   bearish candle before entering short positions
    /// - Particularly effective at resistance levels or after extended uptrends
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (95.0, 100.0, 94.0, 99.0, 0.0); // bullish, body [95, 99]
    /// let curr = (97.0, 97.5, 96.5, 97.0, 0.0); // doji inside prior body
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_bearish_harami_cross());
    /// ```
    pub fn is_bearish_harami_cross(&self) -> bool {
        self.get().zip(self.prev(1)).is_some_and(|(c, p)| {
            let upper = c.open().max(c.close());
            let lower = c.open().min(c.close());
            p.is_bullish() && c.is_doji() && upper < p.close() && lower > p.open()
        })
    }

    /// Identifies a Dark Cloud Cover pattern, a bearish reversal signal in uptrends.
    ///
    /// This two-candle pattern occurs when a bearish candle opens above the prior bullish candle's close
    /// but closes below the midpoint of the prior candle's body. It shows rejection of higher prices.
    ///
    /// **Trading Significance**:
    /// - Signals strong selling pressure after an uptrend
    /// - More significant when the bearish candle closes deep into the prior bullish candle
    /// - Often used by traders to exit long positions or initiate short positions
    /// - Particularly effective when appearing at historical resistance levels
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (100.0, 105.0, 99.5, 104.5, 0.0);
    /// let curr = (105.5, 106.0, 102.0, 101.5, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_dark_cloud_cover());
    /// ```
    pub fn is_dark_cloud_cover(&self) -> bool {
        self.get().zip(self.prev(1)).is_some_and(|(c, p)| {
            c.is_bearish()
                && p.is_bullish()
                && c.open() > p.close()
                && c.close() < midpoint(p.open(), p.close())
        })
    }

    /// Identifies a Piercing Line pattern, the bullish mirror of Dark Cloud Cover.
    ///
    /// This two-candle pattern occurs when a bullish candle opens below the prior
    /// bearish candle's close but closes above the midpoint of the prior candle's body
    /// (while still remaining below the prior open). It signals rejection of lower
    /// prices after a downtrend session.
    ///
    /// **Trading Significance**:
    /// - Signals strong buying pressure after a downtrend
    /// - More significant when the bullish candle closes deeper into the prior bearish body
    /// - Often used by traders to exit short positions or initiate long positions
    /// - Particularly effective when appearing at historical support levels
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (100.0, 102.0, 95.0, 96.0, 0.0); // bearish, midpoint of body = 98
    /// let curr = (94.0, 99.5, 93.0, 99.0, 0.0);   // bullish, closes above 98, below 100
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_piercing_line());
    /// ```
    pub fn is_piercing_line(&self) -> bool {
        self.get().zip(self.prev(1)).is_some_and(|(c, p)| {
            c.is_bullish()
                && p.is_bearish()
                && c.open() < p.close()
                && c.close() > midpoint(p.open(), p.close())
                && c.close() < p.open()
        })
    }

    /// Identifies a Tweezer Top pattern, a bearish reversal signal.
    ///
    /// This two-candle pattern occurs when a bullish candle is followed by a bearish
    /// candle whose **high** matches the prior candle's high within
    /// [`CandleStick::tweezer_tolerance`]. The matching highs form a "tweezer" that
    /// rejects a key resistance level from two directions.
    ///
    /// **Trading Significance**:
    /// - Signals exhaustion of buying pressure at a tested resistance level
    /// - Tolerance is normalised against the larger of the two ranges, so the
    ///   pattern fires reliably across timeframes without manual tuning
    /// - Most effective at established resistance, supply zones, or after an
    ///   extended advance
    /// - Often followed by a measurable pullback within the next 1–3 sessions
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (100.0, 105.0, 99.0, 104.0, 0.0); // bullish, high 105
    /// let curr = (104.5, 105.0, 100.0, 100.5, 0.0); // bearish, high 105
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_tweezer_top());
    /// ```
    pub fn is_tweezer_top(&self) -> bool {
        self.get().zip(self.prev(1)).is_some_and(|(c, p)| {
            let denom = p.range().max(c.range());
            p.is_bullish()
                && c.is_bearish()
                && (p.high() - c.high()).abs() / denom < c.tweezer_tolerance()
        })
    }

    /// Identifies a Tweezer Bottom pattern, the bullish mirror of Tweezer Top.
    ///
    /// This two-candle pattern occurs when a bearish candle is followed by a bullish
    /// candle whose **low** matches the prior candle's low within
    /// [`CandleStick::tweezer_tolerance`]. The matching lows form a "tweezer" that
    /// rejects a key support level from two directions.
    ///
    /// **Trading Significance**:
    /// - Signals exhaustion of selling pressure at a tested support level
    /// - Tolerance is normalised against the larger of the two ranges, so the
    ///   pattern fires reliably across timeframes without manual tuning
    /// - Most effective at established support, demand zones, or after an
    ///   extended decline
    /// - Often followed by a measurable bounce within the next 1–3 sessions
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (100.0, 101.0, 95.0, 96.0, 0.0); // bearish, low 95
    /// let curr = (96.5, 101.0, 95.0, 100.0, 0.0); // bullish, low 95
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_tweezer_bottom());
    /// ```
    pub fn is_tweezer_bottom(&self) -> bool {
        self.get().zip(self.prev(1)).is_some_and(|(c, p)| {
            let denom = p.range().max(c.range());
            p.is_bearish()
                && c.is_bullish()
                && (p.low() - c.low()).abs() / denom < c.tweezer_tolerance()
        })
    }

    /// Identifies an Evening Star pattern, a bearish reversal formation at market tops.
    ///
    /// This three-candle pattern consists of:
    /// 1. A strong bullish candle extending the uptrend
    /// 2. A small-bodied candle showing indecision (star), often with a gap
    /// 3. A bearish candle closing well into the first candle's body
    ///
    /// **Trading Significance**:
    /// - Represents a complete shift from bullish to bearish sentiment
    /// - Considered one of the most reliable bearish reversal patterns
    /// - Traders often exit longs or enter shorts when the third candle confirms
    /// - Effectiveness increases with the size of the third bearish candle
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (100.0, 106.0, 99.5, 105.5, 0.0);
    /// let prev1 = (106.2, 107.0, 105.8, 106.5, 0.0);
    /// let curr = (105.5, 106.0, 102.0, 101.5, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_evening_star());
    /// ```
    pub fn is_evening_star(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bullish()
                    && (p1.is_doji() || p1.open() < p1.close())
                    && c.is_bearish()
                    && c.close() < midpoint(p2.open(), p2.close())
            })
    }

    /// Identifies an Evening Star Doji variant, a strong bearish reversal pattern at market tops.
    ///
    /// This three-candle pattern is similar to the Evening Star, but the middle candle is specifically
    /// a Doji (open ≈ close), emphasizing the perfect equilibrium between buyers and sellers before
    /// bears take control.
    ///
    /// **Trading Significance**:
    /// - Considered stronger than the standard Evening Star due to the Doji's stronger indecision signal
    /// - Often precedes significant price declines when confirmed by the third candle
    /// - Used by traders as a high-probability signal to exit long positions
    /// - Particularly powerful when occurring after an extended uptrend with high momentum
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 =  (100.0, 106.0, 99.5, 105.5, 0.0);
    /// let prev1 =  (106.1, 107.0, 105.8, 106.1, 0.0);
    /// let curr = (105.0, 105.2, 99.8, 101.0, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_evening_star_doji());
    /// ```
    pub fn is_evening_star_doji(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bullish()
                    && p1.is_doji() & c.is_bearish()
                    && c.close() < midpoint(p2.open(), p2.close())
            })
    }

    /// Identifies a Morning Star pattern, a bullish reversal formation at market bottoms.
    ///
    /// This three-candle pattern consists of:
    /// 1. A strong bearish candle extending the downtrend
    /// 2. A small-bodied candle showing indecision (star), often with a gap
    /// 3. A bullish candle closing well into the first candle's body
    ///
    /// **Trading Significance**:
    /// - Represents a complete shift from bearish to bullish sentiment
    /// - Considered one of the most reliable bullish reversal patterns
    /// - Traders often enter long positions when the third candle confirms
    /// - Effectiveness increases with the size of the third bullish candle and supporting volume
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (52.0, 52.5, 48.0, 48.5, 0.0);
    /// let prev1 = (48.2, 48.9, 47.5, 48.3, 0.0);
    /// let curr = (48.7, 51.5, 48.5, 51.2, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_morning_star());
    /// ```
    pub fn is_morning_star(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bearish()
                    && (p1.is_doji() || p1.open() < p1.close())
                    && c.is_bullish()
                    && c.close() > midpoint(p2.open(), p2.close())
            })
    }

    /// Identifies a Morning Star Doji variant, a strong bullish reversal pattern at market bottoms.
    ///
    /// This three-candle pattern is similar to the Morning Star, but the middle candle is specifically
    /// a Doji (open ≈ close), emphasizing the perfect equilibrium between buyers and sellers before
    /// bulls take control.
    ///
    /// **Trading Significance**:
    /// - Considered stronger than the standard Morning Star due to the Doji's stronger indecision signal
    /// - Often precedes significant price rallies when confirmed by the third candle
    /// - Used by traders as a high-probability entry point for long positions
    /// - Particularly powerful when occurring at support levels with increasing volume
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (52.0, 52.5, 48.0, 48.5, 0.0);
    /// let prev1 = (48.3, 48.9, 47.5, 48.4, 0.0);
    /// let curr =  (48.7, 51.5, 48.5, 51.2, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_morning_star_doji());
    /// ```
    pub fn is_morning_star_doji(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bearish()
                    && p1.is_doji()
                    && c.is_bullish()
                    && c.close() > midpoint(p2.open(), p2.close())
            })
    }

    /// Identifies a Bullish Abandoned Baby, the rarest and strongest bullish reversal pattern.
    ///
    /// This three-candle pattern is a strict variant of Morning Star Doji where the
    /// middle Doji is completely isolated by **true price gaps** on both sides:
    /// - C1 is bearish.
    /// - C2 is a Doji whose **entire range** sits below C1's low (`C2.H < C1.L`).
    /// - C3 is bullish and **entirely above** C2's high (`C3.L > C2.H`).
    ///
    /// The doji's full isolation makes this the most uncommon and significant
    /// candlestick reversal — sellers gave up below the prior session entirely, then
    /// buyers seized control with a gap higher.
    ///
    /// **Trading Significance**:
    /// - One of the rarest reversal patterns in liquid markets — most often appears in
    ///   gapping individual equities or open-by-open futures rolls
    /// - When it does appear, it is among the highest-conviction reversal signals
    /// - Traders enter long aggressively, often without waiting for further confirmation
    /// - Particularly effective at major support levels or after extended downtrends
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (100.0, 102.0, 95.0, 96.0, 0.0); // bearish, low 95
    /// let prev1 = (90.0, 90.5, 89.5, 90.0, 0.0);   // doji, high 90.5 < 95
    /// let curr  = (94.0, 100.0, 93.5, 99.0, 0.0);  // bullish, low 93.5 > 90.5
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_bullish_abandoned_baby());
    /// ```
    pub fn is_bullish_abandoned_baby(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bearish()
                    && p1.is_doji()
                    && p1.high() < p2.low()
                    && c.is_bullish()
                    && c.low() > p1.high()
            })
    }

    /// Identifies a Bearish Abandoned Baby, the rarest and strongest bearish reversal pattern.
    ///
    /// Mirror of the Bullish Abandoned Baby:
    /// - C1 is bullish.
    /// - C2 is a Doji whose **entire range** sits above C1's high (`C2.L > C1.H`).
    /// - C3 is bearish and **entirely below** C2's low (`C3.H < C2.L`).
    ///
    /// **Trading Significance**:
    /// - One of the rarest reversal patterns in liquid markets
    /// - When it does appear, it is among the highest-conviction bearish signals
    /// - Traders exit longs and enter shorts aggressively, often without waiting for
    ///   further confirmation
    /// - Particularly effective at major resistance levels or after extended uptrends
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (95.0, 100.0, 94.0, 99.0, 0.0);     // bullish, high 100
    /// let prev1 = (105.0, 105.5, 104.5, 105.0, 0.0);  // doji, low 104.5 > 100
    /// let curr  = (100.0, 102.0, 95.0, 96.0, 0.0);    // bearish, high 102 < 104.5
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_bearish_abandoned_baby());
    /// ```
    pub fn is_bearish_abandoned_baby(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bullish()
                    && p1.is_doji()
                    && p1.low() > p2.high()
                    && c.is_bearish()
                    && c.high() < p1.low()
            })
    }

    /// Identifies a Bullish Tri-Star pattern, a rare three-Doji reversal at market bottoms.
    ///
    /// This three-candle pattern forms at the end of a downtrend and requires:
    /// - All three candles are Dojis.
    /// - The middle Doji's body sits strictly below the outer two Dojis' bodies
    ///   (`max(C2.O, C2.C) < min(C1.O, C1.C, C3.O, C3.C)`).
    ///
    /// Three consecutive Dojis already signal extreme indecision; the middle Doji
    /// gapping below the outer ones turns it into a reversal signal: the market tried
    /// to push lower and failed.
    ///
    /// **Trading Significance**:
    /// - Among the rarest reversal patterns — three exact Dojis is unusual on its own
    /// - When it appears, it is treated as a high-conviction bullish reversal
    /// - Traders typically wait for a fourth bullish candle before entering long
    /// - Most effective at well-established support after a downtrend
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (100.0, 102.0, 98.0, 100.0, 0.0); // doji
    /// let prev1 = (95.0, 96.0, 94.0, 95.0, 0.0);    // doji, body below outer dojis
    /// let curr  = (101.0, 103.0, 99.0, 101.0, 0.0); // doji
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_bullish_tri_star());
    /// ```
    pub fn is_bullish_tri_star(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                let p1_upper = p1.open().max(p1.close());
                let outer_lower = p2
                    .open()
                    .min(p2.close())
                    .min(c.open().min(c.close()));
                p2.is_doji() && p1.is_doji() && c.is_doji() && p1_upper < outer_lower
            })
    }

    /// Identifies a Bearish Tri-Star pattern, a rare three-Doji reversal at market tops.
    ///
    /// Mirror of the Bullish Tri-Star:
    /// - All three candles are Dojis.
    /// - The middle Doji's body sits strictly above the outer two Dojis' bodies
    ///   (`min(C2.O, C2.C) > max(C1.O, C1.C, C3.O, C3.C)`).
    ///
    /// **Trading Significance**:
    /// - Among the rarest reversal patterns
    /// - Treated as a high-conviction bearish reversal when it appears
    /// - Traders typically wait for a fourth bearish candle before entering short
    /// - Most effective at well-established resistance after an uptrend
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (100.0, 102.0, 98.0, 100.0, 0.0); // doji
    /// let prev1 = (105.0, 106.0, 104.0, 105.0, 0.0); // doji, body above outer dojis
    /// let curr  = (99.0, 101.0, 97.0, 99.0, 0.0);   // doji
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_bearish_tri_star());
    /// ```
    pub fn is_bearish_tri_star(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                let p1_lower = p1.open().min(p1.close());
                let outer_upper = p2
                    .open()
                    .max(p2.close())
                    .max(c.open().max(c.close()));
                p2.is_doji() && p1.is_doji() && c.is_doji() && p1_lower > outer_upper
            })
    }

    /// Identifies Three White Soldiers, a powerful bullish reversal or continuation pattern.
    ///
    /// This three-candle pattern consists of consecutive bullish candles, each opening within the previous
    /// candle's body and closing higher, creating a stair-step appearance. Each candle shows progressively
    /// stronger buying pressure overtaking sellers.
    ///
    /// **Trading Significance**:
    /// - Indicates sustained buying pressure and strong bullish momentum
    /// - Shows buyers controlling the market over multiple time periods
    /// - Traders use it to confirm bullish trend reversals or continuations
    /// - Most reliable when candles have minimal upper shadows (little selling pressure at highs)
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (48.0, 50.5, 47.8, 50.2, 0.0);
    /// let prev1 = (50.3, 52.7, 50.1, 52.4, 0.0);
    /// let curr =  (52.5, 54.8, 52.3, 54.5, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_three_white_soldiers());
    /// ```
    pub fn is_three_white_soldiers(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bullish()
                    && p1.is_bullish()
                    && p1.open() > p2.close()
                    && p1.close() > p2.close()
                    && c.is_bullish()
                    && c.open() > p1.close()
                    && c.close() > p1.close()
            })
    }

    /// Identifies Three Black Crows, a powerful bearish reversal or continuation pattern.
    ///
    /// This three-candle pattern consists of consecutive bearish candles, each opening within the previous
    /// candle's body and closing lower, creating a downward stair-step appearance. Each candle shows progressively
    /// stronger selling pressure overtaking buyers.
    ///
    /// **Trading Significance**:
    /// - Indicates sustained selling pressure and strong bearish momentum
    /// - Shows sellers controlling the market over multiple time periods
    /// - Traders use it to confirm bearish trend reversals or continuations
    /// - Most reliable when candles have minimal lower shadows (little buying pressure at lows)
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (54.0, 54.5, 51.8, 52.2, 0.0);
    /// let prev1 = (52.0, 52.3, 49.7, 50.4, 0.0);
    /// let curr =  (50.2, 50.5, 47.9, 48.3, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_three_black_crows());
    /// ```
    pub fn is_three_black_crows(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bearish()
                    && p1.is_bearish()
                    && p1.open() < p2.close()
                    && p1.close() < p2.close()
                    && c.is_bearish()
                    && c.open() < p1.close()
                    && c.close() < p1.close()
            })
    }

    /// Identifies a Bullish Three Line Strike, a four-candle continuation/reversal pattern.
    ///
    /// This pattern combines a Three White Soldiers setup with an immediate counter-strike:
    /// - The three prior candles (C1, C2, C3) are bullish, each closing higher and
    ///   opening above the previous close — i.e. they meet the Three White Soldiers
    ///   criteria.
    /// - C4 (the strike) is bearish, opens above C3's close, and closes below C1's
    ///   open — fully engulfing the prior three bodies.
    ///
    /// Despite its bearish appearance, classical Japanese technical analysis treats
    /// this as a **bullish continuation** signal — the strike is interpreted as
    /// profit-taking that exhausts in a single session, with the underlying trend
    /// resuming.
    ///
    /// **Trading Significance**:
    /// - Classical interpretation: bullish continuation after a violent shakeout
    /// - Western/quant interpretation: often used as a bearish reversal signal
    /// - Either way, the pattern marks a high-volatility decision point on the chart
    /// - Most reliable in trending markets with clear momentum
    /// - Traders watch the close after C4 for confirmation in either direction
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let c1 = (100.0, 105.0, 99.5, 104.0, 0.0);
    /// let c2 = (105.0, 110.0, 104.5, 109.0, 0.0);
    /// let c3 = (110.0, 115.0, 109.5, 114.0, 0.0);
    /// let c4 = (116.0, 117.0, 99.0, 99.5, 0.0); // bearish, opens > c3.close, closes < c1.open
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&c1).push(&c2).push(&c3).push(&c4).is_bullish_three_line_strike());
    /// ```
    pub fn is_bullish_three_line_strike(&self) -> bool {
        match (self.get(), self.prev(1), self.prev(2), self.prev(3)) {
            (Some(c4), Some(c3), Some(c2), Some(c1)) => {
                c1.is_bullish()
                    && c2.is_bullish()
                    && c3.is_bullish()
                    && c1.close() < c2.close()
                    && c2.close() < c3.close()
                    && c2.open() > c1.close()
                    && c3.open() > c2.close()
                    && c4.is_bearish()
                    && c4.open() > c3.close()
                    && c4.close() < c1.open()
            }
            _ => false,
        }
    }

    /// Identifies a Bearish Three Line Strike, the mirror of the bullish variant.
    ///
    /// This pattern combines a Three Black Crows setup with an immediate counter-strike:
    /// - The three prior candles (C1, C2, C3) are bearish, each closing lower and
    ///   opening below the previous close.
    /// - C4 (the strike) is bullish, opens below C3's close, and closes above C1's
    ///   open — fully engulfing the prior three bodies.
    ///
    /// Classical Japanese analysis treats this as a **bearish continuation** signal
    /// — the strike is interpreted as short-covering that exhausts in a single
    /// session, with the underlying trend resuming.
    ///
    /// **Trading Significance**:
    /// - Classical interpretation: bearish continuation after a violent short-cover
    /// - Western/quant interpretation: often used as a bullish reversal signal
    /// - Either way, the pattern marks a high-volatility decision point on the chart
    /// - Most reliable in trending markets with clear momentum
    /// - Traders watch the close after C4 for confirmation in either direction
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let c1 = (100.0, 100.5, 95.0, 96.0, 0.0);
    /// let c2 = (95.0, 95.5, 91.0, 92.0, 0.0);
    /// let c3 = (91.0, 91.5, 87.0, 88.0, 0.0);
    /// let c4 = (86.0, 101.0, 85.5, 101.0, 0.0); // bullish, opens < c3.close, closes > c1.open
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&c1).push(&c2).push(&c3).push(&c4).is_bearish_three_line_strike());
    /// ```
    pub fn is_bearish_three_line_strike(&self) -> bool {
        match (self.get(), self.prev(1), self.prev(2), self.prev(3)) {
            (Some(c4), Some(c3), Some(c2), Some(c1)) => {
                c1.is_bearish()
                    && c2.is_bearish()
                    && c3.is_bearish()
                    && c1.close() > c2.close()
                    && c2.close() > c3.close()
                    && c2.open() < c1.close()
                    && c3.open() < c2.close()
                    && c4.is_bullish()
                    && c4.open() < c3.close()
                    && c4.close() > c1.open()
            }
            _ => false,
        }
    }

    /// Identifies an Advance Block pattern, a bearish exhaustion variant of Three White Soldiers.
    ///
    /// This three-candle pattern resembles Three White Soldiers but with signs of
    /// weakening on each successive candle:
    /// - All three candles are bullish with each closing higher than the previous
    ///   (`C1.C < C2.C < C3.C`).
    /// - Each opens above the previous close (white-soldiers-like).
    /// - Bodies become progressively smaller (`body(C1) > body(C2) > body(C3)`).
    /// - Upper shadows grow on each candle, showing sellers stepping in higher up
    ///   (`wick(C2) > wick(C1)` and `wick(C3) > wick(C2)`).
    ///
    /// The advance is still up, but the conviction is fading visibly bar by bar.
    ///
    /// **Trading Significance**:
    /// - Treated as a bearish exhaustion signal at the end of uptrends
    /// - The growing upper shadows and shrinking bodies are the key tells
    /// - Traders typically use it to tighten stops on longs or take partial profits
    ///   rather than to initiate aggressive shorts
    /// - More reliable when appearing into resistance after an extended advance
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let c1 = (100.0, 105.0, 99.5, 104.0, 0.0); // body 4, wick 1
    /// let c2 = (105.0, 108.5, 104.5, 107.0, 0.0); // body 2, wick 1.5
    /// let c3 = (108.0, 112.0, 107.5, 109.0, 0.0); // body 1, wick 3
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&c1).push(&c2).push(&c3).is_advance_block());
    /// ```
    pub fn is_advance_block(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c3, c2), c1)| {
                c1.is_bullish()
                    && c2.is_bullish()
                    && c3.is_bullish()
                    && c1.close() < c2.close()
                    && c2.close() < c3.close()
                    && c2.open() > c1.close()
                    && c3.open() > c2.close()
                    && c1.body() > c2.body()
                    && c2.body() > c3.body()
                    && c2.wick() > c1.wick()
                    && c3.wick() > c2.wick()
            })
    }

    /// Identifies a Deliberation (also called Stalled) pattern, a bearish exhaustion signal.
    ///
    /// This three-candle pattern resembles Three White Soldiers but with a final
    /// small-bodied bullish candle that hints at fading momentum:
    /// - C1, C2 are long-bodied bullish candles (`body / range > 1 - marubozu_ratio`).
    /// - `C2.C > C1.C` — the second extends the advance.
    /// - C3 is bullish but small-bodied (within [`CandleStick::spinning_top_body_ratio`]).
    /// - `C3.O >= C2.C` and `C3.C > C2.C` — opens at-or-above and closes only
    ///   marginally higher.
    ///
    /// Unlike Advance Block (which fades through shrinking bodies and growing
    /// shadows), Deliberation fades through a single small candle on top — the
    /// market "deliberates" rather than reversing outright.
    ///
    /// **Trading Significance**:
    /// - Bearish exhaustion signal but not a hard reversal trigger
    /// - The small final body says the prior momentum has stalled, not flipped
    /// - Traders use it to tighten stops, take partial profits, or wait for a
    ///   confirming bearish candle before shorting
    /// - More reliable when appearing into resistance or after an extended trend
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let c1 = (100.0, 105.0, 99.5, 104.5, 0.0); // body/range ≈ 0.82
    /// let c2 = (105.0, 110.0, 104.5, 109.5, 0.0); // body/range ≈ 0.82, c2.c > c1.c
    /// let c3 = (110.0, 112.0, 109.0, 110.5, 0.0); // small body, c3.o >= c2.c, c3.c marginally > c2.c
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&c1).push(&c2).push(&c3).is_deliberation());
    /// ```
    pub fn is_deliberation(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c3, c2), c1)| {
                let long_body_threshold = 1.0 - c1.marubozu_ratio();
                c1.is_bullish()
                    && c2.is_bullish()
                    && c1.body_range_ratio() > long_body_threshold
                    && c2.body_range_ratio() > long_body_threshold
                    && c2.close() > c1.close()
                    && c3.is_bullish()
                    && c3.body_range_ratio() < c3.spinning_top_body_ratio()
                    && c3.open() >= c2.close()
                    && c3.close() > c2.close()
            })
    }

    /// Identifies the Three Inside Up pattern, a bullish reversal.
    ///
    /// This three-candle pattern typically appears in a downtrend and signals a potential
    /// shift from bearish to bullish momentum. This pattern can be seen as a confirmation or
    /// continuation of a bullish harami.
    ///
    /// **Trading Significance**:
    /// - Suggests that prior selling pressure is weakening
    /// - Indicates buyers are starting to regain control
    /// - Often used as an early sign of a bullish reversal after a down move
    /// - Considerably stronger when combined with support levels, volume confirmation,
    ///   or higher-timeframe confluence
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    ///
    /// let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
    /// let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);
    /// let curr  = (52.9, 55.0, 52.7, 54.5, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_three_inside_up());
    /// ```
    pub fn is_three_inside_up(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bearish()
                    && p1.is_bullish()
                    && p1.open() > p2.close()
                    && p1.close() < p2.open()
                    && c.is_bullish()
                    && c.close() > p1.close()
                    && !c.is_doji()
            })
    }

    /// Identifies the Three Inside Down pattern, a bearish reversal.
    ///
    /// This three-candle pattern typically appears in an uptrend and signals a potential
    /// shift from bullish to bearish momentum. This pattern can be seen as a confirmation or
    /// continuation of a bearish harami.
    ///
    /// **Trading Significance**:
    /// - Suggests that prior buying pressure is weakening
    /// - Indicates sellers are starting to regain control
    /// - Often used as an early sign of a bearish reversal after an up move
    /// - Considerably stronger when combined with resistance levels, volume confirmation,
    ///   or higher-timeframe confluence
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
    /// let prev1 = (49.5, 49.8, 48.5, 49.0, 0.0);
    /// let curr  = (48.8, 49.0, 47.5, 47.9, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_three_inside_down());
    /// ```
    pub fn is_three_inside_down(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bullish()
                    && p1.is_bearish()
                    && p1.open() < p2.close()
                    && p1.close() > p2.open()
                    && c.is_bearish()
                    && c.close() < p1.close()
                    && !c.is_doji()
            })
    }

    /// Identifies the Three Outside Up pattern, a bullish reversal.
    ///
    /// This three-candle pattern is the engulfing-based companion to Three Inside Up:
    /// - C1 is bearish.
    /// - C2 is bullish and engulfs C1's body (`C2.O < C1.C` and `C2.C > C1.O`) —
    ///   matching the Bullish Engulfing criteria.
    /// - C3 is bullish and closes higher than C2 (`C3.C > C2.C`), confirming the
    ///   engulfing rejection.
    ///
    /// Where Three Inside Up confirms a Bullish Harami, Three Outside Up confirms a
    /// Bullish Engulfing — and is therefore generally treated as a stronger reversal
    /// signal due to the larger directional move on C2.
    ///
    /// **Trading Significance**:
    /// - Stronger than Three Inside Up because the C1→C2 engulfing already carries
    ///   high conviction
    /// - C3 acts as confirmation that the reversal has follow-through
    /// - Most effective at support levels or after extended downtrends
    /// - Volume confirmation on C2 and C3 increases reliability
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (100.0, 101.0, 95.0, 96.0, 0.0); // bearish, body [96, 100]
    /// let prev1 = (95.0, 105.0, 94.5, 104.0, 0.0); // bullish, engulfs prev2's body
    /// let curr  = (104.0, 108.0, 103.0, 107.0, 0.0); // bullish, closes higher
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_three_outside_up());
    /// ```
    pub fn is_three_outside_up(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bearish()
                    && p1.is_bullish()
                    && p1.open() < p2.close()
                    && p1.close() > p2.open()
                    && c.is_bullish()
                    && c.close() > p1.close()
            })
    }

    /// Identifies the Three Outside Down pattern, a bearish reversal.
    ///
    /// Mirror of Three Outside Up — the engulfing-based companion to Three Inside Down:
    /// - C1 is bullish.
    /// - C2 is bearish and engulfs C1's body (`C2.O > C1.C` and `C2.C < C1.O`).
    /// - C3 is bearish and closes lower than C2 (`C3.C < C2.C`).
    ///
    /// **Trading Significance**:
    /// - Stronger than Three Inside Down because the C1→C2 engulfing already carries
    ///   high conviction
    /// - C3 acts as confirmation that the reversal has follow-through
    /// - Most effective at resistance levels or after extended uptrends
    /// - Volume confirmation on C2 and C3 increases reliability
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (95.0, 100.0, 94.0, 99.0, 0.0);  // bullish, body [95, 99]
    /// let prev1 = (100.0, 101.0, 90.0, 91.0, 0.0); // bearish, engulfs prev2's body
    /// let curr  = (91.0, 92.0, 87.0, 88.0, 0.0);   // bearish, closes lower
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_three_outside_down());
    /// ```
    pub fn is_three_outside_down(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bullish()
                    && p1.is_bearish()
                    && p1.open() > p2.close()
                    && p1.close() < p2.open()
                    && c.is_bearish()
                    && c.close() < p1.close()
            })
    }

    /// Identifies a downtrend from the last 3 candles.
    /// A downtrend is defined as a series of strictly lower highs and lower lows.
    pub fn is_downtrend(&self) -> bool {
        match (self.get(), self.prev(1), self.prev(2)) {
            (Some(c0), Some(c1), Some(c2)) => {
                (c0.high() < c1.high() && c1.high() < c2.high())
                    && (c0.low() < c1.low() && c1.low() < c2.low())
            }
            _ => false,
        }
    }

    /// Identifies an uptrend from the last 3 candles.
    /// An uptrend is defined as a series of strictly higher highs and higher lows.
    pub fn is_uptrend(&self) -> bool {
        match (self.get(), self.prev(1), self.prev(2)) {
            (Some(c0), Some(c1), Some(c2)) => {
                (c0.high() > c1.high() && c1.high() > c2.high())
                    && (c0.low() > c1.low() && c1.low() > c2.low())
            }
            _ => false,
        }
    }
}

impl<T> Default for CandleStream<T> {
    fn default() -> Self {
        Self {
            series: [const { None }; SERIES_SIZE],
            idx: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_index() {
        let candle1 = (100.0, 105.0, 99.0, 104.0, 0.0);
        let candle2 = (104.5, 110.0, 104.0, 109.0, 0.0);
        let candle3 = (109.5, 112.0, 108.0, 111.0, 0.0);
        let candle4 = (111.5, 115.0, 110.0, 114.0, 0.0);
        let candle5 = (114.5, 118.0, 113.0, 117.0, 0.0);
        let candle6 = (117.5, 120.0, 116.0, 119.0, 0.0);

        let mut stream = CandleStream::new();

        assert_eq!(stream.nth_index(6), None);

        stream.push(&candle1).push(&candle2);

        assert_eq!(stream.nth_index(0), Some(2));
        assert_eq!(stream.nth_index(1), Some(1));
        assert_eq!(stream.nth_index(2), Some(0));

        stream.push(&candle3).push(&candle4).push(&candle5);

        assert_eq!(stream.nth_index(0), Some(0));
        assert_eq!(stream.nth_index(1), Some(4));
        assert_eq!(stream.nth_index(2), Some(3));
        assert_eq!(stream.nth_index(3), Some(2));
        assert_eq!(stream.nth_index(4), Some(1));
        assert_eq!(stream.nth_index(5), Some(0));

        stream.push(&candle6);

        assert_eq!(stream.nth_index(0), Some(1));
        assert_eq!(stream.nth_index(1), Some(0));
        assert_eq!(stream.nth_index(2), Some(4));
        assert_eq!(stream.nth_index(3), Some(3));
        assert_eq!(stream.nth_index(4), Some(2));
        assert_eq!(stream.nth_index(5), Some(1));
    }

    #[test]
    fn test_at() {
        let candle1 = (100.0, 105.0, 99.0, 104.0, 0.0);
        let candle2 = (104.5, 110.0, 104.0, 109.0, 0.0);
        let candle3 = (109.5, 112.0, 108.0, 111.0, 0.0);
        let candle4 = (111.5, 115.0, 110.0, 114.0, 0.0);
        let candle5 = (114.5, 118.0, 113.0, 117.0, 0.0);
        let candle6 = (117.5, 120.0, 116.0, 119.0, 0.0);

        let mut stream = CandleStream::new();
        stream.push(candle1).push(candle2);

        assert_eq!(stream.at(0), Some(&candle1));
        assert_eq!(stream.at(1), Some(&candle2));
        assert_eq!(stream.at(2), None);

        stream.push(candle3).push(candle4).push(candle5);

        assert_eq!(stream.at(0), Some(&candle1));
        assert_eq!(stream.at(1), Some(&candle2));
        assert_eq!(stream.at(2), Some(&candle3));
        assert_eq!(stream.at(3), Some(&candle4));
        assert_eq!(stream.at(4), Some(&candle5));

        stream.push(candle6);

        assert_eq!(stream.at(0), Some(&candle6));
        assert_eq!(stream.at(1), Some(&candle2));
        assert_eq!(stream.at(2), Some(&candle3));
        assert_eq!(stream.at(3), Some(&candle4));
        assert_eq!(stream.at(4), Some(&candle5));
    }

    #[test]
    fn test_get() {
        let candle1 = (100.0, 105.0, 99.0, 104.0, 0.0);
        let candle2 = (104.5, 110.0, 104.0, 109.0, 0.0);
        let candle3 = (109.5, 112.0, 108.0, 111.0, 0.0);

        let mut stream = CandleStream::new();
        assert_eq!(stream.get(), None);

        stream.push(candle1);
        assert_eq!(stream.get(), Some(&candle1));

        stream.push(candle2);
        assert_eq!(stream.get(), Some(&candle2));

        stream.push(candle3).push(candle1).push(candle2);
        assert_eq!(stream.get(), Some(&candle2));

        stream.push(candle3);
        assert_eq!(stream.get(), Some(&candle3));
    }

    #[test]
    fn test_prev() {
        let candle1 = (100.0, 105.0, 99.0, 104.0, 0.0);
        let candle2 = (104.5, 110.0, 104.0, 109.0, 0.0);
        let candle3 = (109.5, 112.0, 108.0, 111.0, 0.0);

        let mut stream = CandleStream::new();
        assert_eq!(stream.prev(1), None);

        stream.push(candle1);
        assert_eq!(stream.prev(1), None);

        stream.push(candle2);
        assert_eq!(stream.prev(1), Some(&candle1));

        stream.push(candle3);
        assert_eq!(stream.prev(1), Some(&candle2));
        assert_eq!(stream.prev(2), Some(&candle1));
    }

    #[test]
    fn test_is_three_inside_up() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);
        let curr = (52.9, 55.0, 52.7, 54.5, 0.0);

        let mut series = CandleStream::new();

        assert!(series
            .push(&prev2)
            .push(&prev1)
            .push(&curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_three_inside_up_if_curr_engulfs_prev1() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);
        let curr_engulf_prev1 = (52.0, 55.0, 51.9, 53.5, 0.0);

        let mut series = CandleStream::new();

        assert!(series
            .push(&prev2)
            .push(&prev1)
            .push(&curr_engulf_prev1)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_curr_is_doji() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);
        let doji = (53.4, 55.0, 52.7, 53.5, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1)
            .push(&doji)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_prev2_not_bearish() {
        let not_bearish_prev2 = (52.0, 54.5, 51.8, 54.0, 0.0);
        let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);
        let curr = (52.9, 55.0, 52.7, 54.5, 0.0); // valid curr

        let mut series = CandleStream::new();

        assert!(!series
            .push(&not_bearish_prev2)
            .push(&prev1)
            .push(&curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_prev2_is_doji() {
        let doji_prev2 = (53.0, 54.5, 51.8, 53.0, 0.0);
        let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);
        let curr = (52.9, 55.0, 52.7, 54.5, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&doji_prev2)
            .push(&prev1)
            .push(&curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_prev1_not_bullish() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let not_bullish_prev1 = (52.8, 53.0, 52.0, 52.2, 0.0); // open > close
        let curr = (52.9, 55.0, 52.7, 54.5, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&not_bullish_prev1)
            .push(&curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_prev1_opens_below_prev2_close() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1_open_below_prev2 = (51.9, 53.0, 51.8, 52.5, 0.0);
        let curr = (52.9, 55.0, 52.7, 54.5, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1_open_below_prev2)
            .push(&curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_prev1_closes_above_prev2_open() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1_close_above_prev2 = (52.2, 55.0, 52.0, 54.5, 0.0);
        let curr = (54.6, 56.0, 52.7, 55.0, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1_close_above_prev2)
            .push(&curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_prev1_engulfs_prev2() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1_engulf_prev2 = (51.5, 55.0, 51.0, 54.5, 0.0);
        let curr = (54.6, 56.0, 53.5, 55.5, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1_engulf_prev2)
            .push(&curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_prev1_is_doji() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let doji_prev1 = (52.8, 53.0, 52.0, 52.8, 0.0);
        let curr = (52.9, 55.0, 52.7, 54.5, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&doji_prev1)
            .push(&curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_curr_is_inside_prev1() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);
        let curr_inside_prev1 = (52.3, 53.1, 52.1, 52.7, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1)
            .push(&curr_inside_prev1)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_curr_not_bullish() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);
        let not_bullish_curr = (55.0, 55.5, 52.7, 53.0, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1)
            .push(&not_bullish_curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_with_insufficient_candles() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);

        let mut series = CandleStream::new();

        assert!(!series.push(&prev2).is_three_inside_up());
        assert!(!series.push(&prev1).is_three_inside_up());
    }

    #[test]
    fn test_is_three_inside_down() {
        let prev2: (f64, f64, f64, f64, f64) = (48.0, 50.5, 47.8, 50.0, 0.0);
        let prev1: (f64, f64, f64, f64, f64) = (49.5, 49.8, 48.5, 49.0, 0.0);
        let curr: (f64, f64, f64, f64, f64) = (48.8, 49.0, 47.5, 47.9, 0.0);

        let mut series: CandleStream<(f64, f64, f64, f64, f64)> = CandleStream::new();

        assert!(series
            .push(prev2)
            .push(prev1)
            .push(curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_three_inside_down_if_curr_engulfs_prev1() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let prev1 = (49.5, 49.8, 48.5, 49.0, 0.0);
        let curr_engulf_prev1 = (49.8, 50.0, 47.5, 48.8, 0.0); // open > prev1.open, close < prev1.close

        let mut series = CandleStream::new();

        assert!(series
            .push(&prev2)
            .push(&prev1)
            .push(&curr_engulf_prev1)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_curr_is_doji() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let prev1 = (49.5, 49.8, 48.5, 49.0, 0.0);
        let doji = (48.5, 50.0, 47.5, 48.5, 0.0); // open == close

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1)
            .push(&doji)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_prev2_not_bullish() {
        let not_bullish_prev2 = (50.0, 50.5, 47.8, 48.0, 0.0); // bearish instead of bullish
        let prev1 = (49.5, 49.8, 48.5, 49.0, 0.0);
        let curr = (48.8, 49.0, 47.5, 47.9, 0.0); // valid curr

        let mut series = CandleStream::new();

        assert!(!series
            .push(&not_bullish_prev2)
            .push(&prev1)
            .push(&curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_prev2_is_doji() {
        let doji_prev2 = (49.0, 50.5, 47.8, 49.0, 0.0); // open == close
        let prev1 = (49.5, 49.8, 48.5, 49.0, 0.0);
        let curr = (48.8, 49.0, 47.5, 47.9, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&doji_prev2)
            .push(&prev1)
            .push(&curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_prev1_not_bearish() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let not_bearish_prev1 = (48.5, 49.5, 48.0, 49.2, 0.0); // open < close
        let curr = (48.8, 49.0, 47.5, 47.9, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&not_bearish_prev1)
            .push(&curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_prev1_opens_above_prev2_close() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let prev1_open_above_prev2 = (50.2, 50.5, 48.5, 49.5, 0.0);
        let curr = (48.8, 49.0, 47.5, 47.9, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1_open_above_prev2)
            .push(&curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_prev1_closes_below_prev2_open() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let prev1_close_below_prev2 = (49.5, 49.8, 47.5, 47.9, 0.0); // close < 48.0
        let curr = (48.8, 49.0, 47.5, 47.9, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1_close_below_prev2)
            .push(&curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_prev1_engulfs_prev2() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0); // body [48.0, 50.0]
        let prev1_engulf_prev2 = (50.5, 51.0, 47.0, 47.5, 0.0); // open > 50.0, close < 48.0
        let curr = (48.8, 49.0, 47.5, 47.9, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1_engulf_prev2)
            .push(&curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_prev1_is_doji() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let doji_prev1 = (49.0, 49.5, 48.5, 49.0, 0.0); // open == close
        let curr = (48.8, 49.0, 47.5, 47.9, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&doji_prev1)
            .push(&curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_curr_is_inside_prev1() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let prev1 = (49.5, 49.8, 48.5, 49.0, 0.0); // body [49.0, 49.5]
        let curr_inside_prev1 = (49.4, 49.6, 48.8, 49.1, 0.0); // close 49.1 > 49.0

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1)
            .push(&curr_inside_prev1)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_curr_not_bearish() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let prev1 = (49.5, 49.8, 48.5, 49.0, 0.0);
        let not_bearish_curr = (47.8, 48.5, 47.5, 48.6, 0.0); // bullish

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1)
            .push(&not_bearish_curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_with_insufficient_candles() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let prev1 = (49.5, 49.8, 48.5, 49.0, 0.0);

        let mut series = CandleStream::new();

        assert!(!series.push(&prev2).is_three_inside_down());
        assert!(!series.push(&prev1).is_three_inside_down());
    }

    #[test]
    fn test_is_downtrend() {
        let prev2 = (109.5, 112.0, 108.0, 111.0, 0.0);
        let prev1 = (104.5, 110.0, 104.0, 109.0, 0.0);
        let curr = (100.0, 105.0, 99.0, 104.0, 0.0);

        let mut series = CandleStream::new();

        assert!(series.push(&prev2).push(&prev1).push(&curr).is_downtrend());
    }

    #[test]
    fn test_is_not_downtrend() {
        let prev2 = (109.5, 112.0, 108.0, 111.0, 0.0);
        let prev1 = (100.0, 105.0, 99.0, 104.0, 0.0);
        let curr = (104.5, 110.0, 104.0, 109.0, 0.0);

        let mut series = CandleStream::new();

        assert!(!series.push(&prev2).push(&prev1).push(&curr).is_downtrend());
    }

    #[test]
    fn test_is_uptrend() {
        let prev2 = (100.0, 105.0, 99.0, 104.0, 0.0);
        let prev1 = (104.5, 110.0, 104.0, 109.0, 0.0);
        let curr = (109.5, 112.0, 108.0, 111.0, 0.0);

        let mut series = CandleStream::new();

        assert!(series.push(&prev2).push(&prev1).push(&curr).is_uptrend());
    }

    #[test]
    fn test_is_not_uptrend() {
        let prev2 = (100.0, 105.0, 99.0, 104.0, 0.0);
        let prev1 = (109.5, 112.0, 108.0, 111.0, 0.0);
        let curr = (104.5, 110.0, 104.0, 109.0, 0.0);

        let mut series = CandleStream::new();

        assert!(!series.push(&prev2).push(&prev1).push(&curr).is_uptrend());
    }

    // ── Piercing Line ───────────────────────────────────────────────────

    #[test]
    fn test_is_not_piercing_line_if_close_below_midpoint() {
        // Close lands below midpoint(prev.open, prev.close) — fails the
        // 50%-pierce requirement that distinguishes Piercing Line from a
        // shallow bounce.
        let prev = (100.0, 102.0, 95.0, 96.0, 0.0); // midpoint = 98
        let curr = (94.0, 97.5, 93.0, 97.0, 0.0); // closes at 97 < 98
        let mut series = CandleStream::new();
        assert!(!series.push(&prev).push(&curr).is_piercing_line());
    }

    #[test]
    fn test_is_not_piercing_line_if_close_above_prev_open() {
        // Close above prev.open would be a Bullish Engulfing, not Piercing.
        let prev = (100.0, 102.0, 95.0, 96.0, 0.0);
        let curr = (94.0, 101.5, 93.0, 101.0, 0.0); // closes at 101 > prev.open = 100
        let mut series = CandleStream::new();
        assert!(!series.push(&prev).push(&curr).is_piercing_line());
    }

    // ── Tweezer Top / Bottom ────────────────────────────────────────────

    #[test]
    fn test_is_not_tweezer_top_if_highs_diverge_beyond_tolerance() {
        let prev = (100.0, 105.0, 99.0, 104.0, 0.0); // high 105, range 6
        let curr = (104.5, 106.0, 100.0, 100.5, 0.0); // high 106 — gap 1.0 / 6 = 0.167 > 0.05
        let mut series = CandleStream::new();
        assert!(!series.push(&prev).push(&curr).is_tweezer_top());
    }

    #[test]
    fn test_is_not_tweezer_top_if_directions_wrong() {
        // Both bullish — Tweezer Top needs prev bullish AND curr bearish.
        let prev = (100.0, 105.0, 99.0, 104.0, 0.0);
        let curr = (102.0, 105.0, 101.0, 104.5, 0.0);
        let mut series = CandleStream::new();
        assert!(!series.push(&prev).push(&curr).is_tweezer_top());
    }

    #[test]
    fn test_is_not_tweezer_bottom_if_lows_diverge_beyond_tolerance() {
        let prev = (100.0, 101.0, 95.0, 96.0, 0.0); // low 95, range 6
        let curr = (96.5, 101.0, 93.5, 100.0, 0.0); // low 93.5 — gap 1.5 / 6 = 0.25 > 0.05
        let mut series = CandleStream::new();
        assert!(!series.push(&prev).push(&curr).is_tweezer_bottom());
    }

    // ── Harami Cross ────────────────────────────────────────────────────

    #[test]
    fn test_is_not_bullish_harami_cross_if_doji_breaches_prev_body() {
        // Doji sits OUTSIDE prev body (open == close == 101 > prev.open 100).
        let prev = (100.0, 100.5, 95.0, 96.0, 0.0);
        let curr = (101.0, 101.5, 100.5, 101.0, 0.0);
        let mut series = CandleStream::new();
        assert!(!series.push(&prev).push(&curr).is_bullish_harami_cross());
    }

    #[test]
    fn test_is_not_bearish_harami_cross_if_curr_not_doji() {
        // Curr is a real bearish candle, not a doji.
        let prev = (95.0, 100.0, 94.0, 99.0, 0.0);
        let curr = (98.0, 98.5, 96.0, 96.5, 0.0); // sizable body, not a doji
        let mut series = CandleStream::new();
        assert!(!series.push(&prev).push(&curr).is_bearish_harami_cross());
    }

    // ── Kicker ──────────────────────────────────────────────────────────

    #[test]
    fn test_is_not_bullish_kicker_if_prev_not_marubozu() {
        // Prev is bearish but has a long lower shadow — not a marubozu.
        let prev = (100.0, 100.0, 90.0, 95.0, 0.0); // body 5, tail 5, tail/body = 1.0 > 0.2
        let curr = (105.0, 110.0, 105.0, 110.0, 0.0);
        let mut series = CandleStream::new();
        assert!(!series.push(&prev).push(&curr).is_bullish_kicker());
    }

    #[test]
    fn test_is_not_bullish_kicker_if_curr_opens_below_prev_open() {
        // Body overlap: curr opens below prev.open → not a true kicker gap.
        let prev = (100.0, 100.0, 95.0, 95.0, 0.0);
        let curr = (99.0, 104.0, 99.0, 104.0, 0.0); // opens 99 < 100
        let mut series = CandleStream::new();
        assert!(!series.push(&prev).push(&curr).is_bullish_kicker());
    }

    #[test]
    fn test_is_not_bearish_kicker_if_curr_opens_above_prev_open() {
        let prev = (95.0, 100.0, 95.0, 100.0, 0.0);
        let curr = (96.0, 96.0, 91.0, 91.0, 0.0); // opens 96 > 95
        let mut series = CandleStream::new();
        assert!(!series.push(&prev).push(&curr).is_bearish_kicker());
    }

    // ── Abandoned Baby ──────────────────────────────────────────────────

    #[test]
    fn test_is_not_bullish_abandoned_baby_if_doji_touches_prev_low() {
        // Doji high == prev low → no gap. Strict version requires strict
        // inequality.
        let prev2 = (100.0, 102.0, 95.0, 96.0, 0.0); // low 95
        let prev1 = (94.5, 95.0, 94.0, 94.5, 0.0); // high 95 == prev2.low 95
        let curr = (94.0, 100.0, 93.5, 99.0, 0.0);
        let mut series = CandleStream::new();
        assert!(!series.push(&prev2).push(&prev1).push(&curr).is_bullish_abandoned_baby());
    }

    #[test]
    fn test_is_not_bullish_abandoned_baby_if_curr_overlaps_doji() {
        // C3.low must be > C2.high. Here C3.low == C2.high.
        let prev2 = (100.0, 102.0, 95.0, 96.0, 0.0);
        let prev1 = (90.0, 90.5, 89.5, 90.0, 0.0);
        let curr = (90.5, 100.0, 90.5, 99.0, 0.0); // low == prev1.high
        let mut series = CandleStream::new();
        assert!(!series.push(&prev2).push(&prev1).push(&curr).is_bullish_abandoned_baby());
    }

    // ── Tri-Star ────────────────────────────────────────────────────────

    #[test]
    fn test_is_not_bullish_tri_star_if_one_candle_not_doji() {
        // Middle candle has a real body.
        let prev2 = (100.0, 102.0, 98.0, 100.0, 0.0);
        let prev1 = (95.0, 96.0, 90.0, 91.0, 0.0); // sizable body, not a doji
        let curr = (101.0, 103.0, 99.0, 101.0, 0.0);
        let mut series = CandleStream::new();
        assert!(!series.push(&prev2).push(&prev1).push(&curr).is_bullish_tri_star());
    }

    #[test]
    fn test_is_not_bullish_tri_star_if_middle_body_overlaps_outer_bodies() {
        // Middle doji's body is at 100 — same as the outer dojis. No gap.
        let prev2 = (100.0, 102.0, 98.0, 100.0, 0.0);
        let prev1 = (100.0, 100.5, 99.5, 100.0, 0.0); // body == outer bodies
        let curr = (100.0, 102.0, 98.0, 100.0, 0.0);
        let mut series = CandleStream::new();
        assert!(!series.push(&prev2).push(&prev1).push(&curr).is_bullish_tri_star());
    }

    // ── Three Line Strike ───────────────────────────────────────────────

    #[test]
    fn test_is_not_bullish_three_line_strike_if_strike_does_not_engulf() {
        // C4 closes at 100.5 — above C1.open (100), so doesn't engulf the
        // three soldiers.
        let c1 = (100.0, 105.0, 99.5, 104.0, 0.0);
        let c2 = (105.0, 110.0, 104.5, 109.0, 0.0);
        let c3 = (110.0, 115.0, 109.5, 114.0, 0.0);
        let c4 = (116.0, 117.0, 100.5, 100.5, 0.0);
        let mut series = CandleStream::new();
        assert!(!series.push(&c1).push(&c2).push(&c3).push(&c4).is_bullish_three_line_strike());
    }

    #[test]
    fn test_is_not_bullish_three_line_strike_with_insufficient_candles() {
        let c1 = (100.0, 105.0, 99.5, 104.0, 0.0);
        let c2 = (105.0, 110.0, 104.5, 109.0, 0.0);
        let c3 = (110.0, 115.0, 109.5, 114.0, 0.0);
        let mut series = CandleStream::new();
        assert!(!series.push(&c1).push(&c2).push(&c3).is_bullish_three_line_strike());
    }

    // ── Advance Block ───────────────────────────────────────────────────

    #[test]
    fn test_is_not_advance_block_if_bodies_grow() {
        // body(C3) > body(C2) — fails the progressive-shrinking criterion.
        let c1 = (100.0, 105.0, 99.5, 104.0, 0.0); // body 4
        let c2 = (105.0, 108.5, 104.5, 107.0, 0.0); // body 2
        let c3 = (108.0, 113.0, 107.5, 112.0, 0.0); // body 4
        let mut series = CandleStream::new();
        assert!(!series.push(&c1).push(&c2).push(&c3).is_advance_block());
    }

    #[test]
    fn test_is_not_advance_block_if_upper_shadow_shrinks() {
        // wick(C3) < wick(C2) — fails the growing-shadow criterion.
        let c1 = (100.0, 105.0, 99.5, 104.0, 0.0); // wick 1
        let c2 = (105.0, 108.5, 104.5, 107.0, 0.0); // wick 1.5
        let c3 = (108.0, 109.5, 107.5, 109.0, 0.0); // wick 0.5
        let mut series = CandleStream::new();
        assert!(!series.push(&c1).push(&c2).push(&c3).is_advance_block());
    }

    // ── Deliberation ────────────────────────────────────────────────────

    #[test]
    fn test_is_not_deliberation_if_c3_body_too_large() {
        // C3 body/range = 0.5 — fails the small-body criterion (< 0.2).
        let c1 = (100.0, 105.0, 99.5, 104.5, 0.0);
        let c2 = (105.0, 110.0, 104.5, 109.5, 0.0);
        let c3 = (110.0, 112.0, 109.0, 111.0, 0.0); // body 1, range 3, ratio 0.33
        let mut series = CandleStream::new();
        assert!(!series.push(&c1).push(&c2).push(&c3).is_deliberation());
    }

    #[test]
    fn test_is_not_deliberation_if_c3_close_below_c2_close() {
        // C3 closes below C2.C — pattern requires marginal advance.
        let c1 = (100.0, 105.0, 99.5, 104.5, 0.0);
        let c2 = (105.0, 110.0, 104.5, 109.5, 0.0);
        let c3 = (110.0, 112.0, 109.0, 109.0, 0.0); // close 109 < 109.5
        let mut series = CandleStream::new();
        assert!(!series.push(&c1).push(&c2).push(&c3).is_deliberation());
    }

    // ── Three Outside Up / Down ─────────────────────────────────────────

    #[test]
    fn test_is_not_three_outside_up_if_c2_does_not_engulf() {
        // C2 is bullish but doesn't engulf C1's body — close above C1.open
        // is missing.
        let prev2 = (100.0, 101.0, 95.0, 96.0, 0.0); // body [96, 100]
        let prev1 = (95.0, 99.5, 94.5, 99.0, 0.0); // close 99 < 100
        let curr = (99.5, 105.0, 99.0, 104.0, 0.0);
        let mut series = CandleStream::new();
        assert!(!series.push(&prev2).push(&prev1).push(&curr).is_three_outside_up());
    }

    #[test]
    fn test_is_not_three_outside_down_if_c3_closes_above_c2() {
        // C3.C > C2.C — the confirmation candle moves the wrong way.
        let prev2 = (95.0, 100.0, 94.0, 99.0, 0.0);
        let prev1 = (100.0, 101.0, 90.0, 91.0, 0.0);
        let curr = (91.0, 95.0, 90.5, 94.0, 0.0); // close 94 > prev1.close 91
        let mut series = CandleStream::new();
        assert!(!series.push(&prev2).push(&prev1).push(&curr).is_three_outside_down());
    }
}
