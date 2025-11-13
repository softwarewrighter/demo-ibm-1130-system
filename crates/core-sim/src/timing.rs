//! Timing model for IBM 1130 device simulation.
//!
//! The timing model allows switching between different timing modes:
//! - **None**: Zero delays for deterministic, fast testing
//! - **Realistic**: 1x historical timing (accurate to IBM specifications)
//! - **Fast**: Accelerated timing for demos and development

/// Timing mode for device simulation.
///
/// This enables three different timing behaviors:
/// - `None`: All operations complete instantly (delay_us returns 0)
/// - `Realistic`: Historical 1x timing matching IBM 1130 specifications
/// - `Fast(n)`: Operations complete n-times faster than realistic
///
/// # Examples
///
/// ```
/// use core_sim::TimingModel;
///
/// // For testing: instant operations
/// let timing = TimingModel::none();
/// assert_eq!(timing.delay_us(1000), 0);
///
/// // For education: realistic timing
/// let timing = TimingModel::realistic();
/// assert_eq!(timing.delay_us(1000), 1000);
///
/// // For demos: 10x faster
/// let timing = TimingModel::fast(10.0);
/// assert_eq!(timing.delay_us(1000), 100);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TimingModel {
    /// No timing delays (instant operations for testing)
    None,
    /// Historical 1x timing matching IBM 1130 specifications
    #[default]
    Realistic,
    /// Accelerated timing (multiplier > 1.0 means faster)
    Fast(f64),
}

impl TimingModel {
    /// Create a timing model with no delays (instant operations).
    ///
    /// Use this for unit tests where deterministic, fast execution is needed.
    pub fn none() -> Self {
        Self::None
    }

    /// Create a timing model with realistic 1x historical timing.
    ///
    /// Use this for educational purposes to show actual IBM 1130 performance.
    pub fn realistic() -> Self {
        Self::Realistic
    }

    /// Create a timing model with accelerated timing.
    ///
    /// # Arguments
    ///
    /// * `multiplier` - Speed multiplier (e.g., 10.0 means 10x faster)
    ///
    /// # Examples
    ///
    /// ```
    /// use core_sim::TimingModel;
    ///
    /// // 50x faster than realistic for quick demos
    /// let timing = TimingModel::fast(50.0);
    /// assert_eq!(timing.delay_us(5000), 100);
    /// ```
    pub fn fast(multiplier: f64) -> Self {
        Self::Fast(multiplier)
    }

    /// Calculate the actual delay in microseconds for a given nominal delay.
    ///
    /// # Arguments
    ///
    /// * `nominal_us` - The historical delay in microseconds
    ///
    /// # Returns
    ///
    /// The adjusted delay based on the timing mode:
    /// - `None`: Always returns 0
    /// - `Realistic`: Returns `nominal_us` unchanged
    /// - `Fast(n)`: Returns `nominal_us / n`
    ///
    /// # Examples
    ///
    /// ```
    /// use core_sim::TimingModel;
    ///
    /// let none = TimingModel::none();
    /// let realistic = TimingModel::realistic();
    /// let fast = TimingModel::fast(2.0);
    ///
    /// assert_eq!(none.delay_us(1000), 0);
    /// assert_eq!(realistic.delay_us(1000), 1000);
    /// assert_eq!(fast.delay_us(1000), 500);
    /// ```
    pub fn delay_us(&self, nominal_us: u64) -> u64 {
        match self {
            TimingModel::None => 0,
            TimingModel::Realistic => nominal_us,
            TimingModel::Fast(multiplier) => ((nominal_us as f64) / multiplier) as u64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timing_none_returns_zero_delay() {
        let timing = TimingModel::none();
        assert_eq!(timing.delay_us(0), 0);
        assert_eq!(timing.delay_us(1000), 0);
        assert_eq!(timing.delay_us(1_000_000), 0);
    }

    #[test]
    fn test_timing_realistic_returns_unchanged_delay() {
        let timing = TimingModel::realistic();
        assert_eq!(timing.delay_us(0), 0);
        assert_eq!(timing.delay_us(1000), 1000);
        assert_eq!(timing.delay_us(27_800), 27_800); // Word transfer time
        assert_eq!(timing.delay_us(22_500), 22_500); // Settle time
    }

    #[test]
    fn test_timing_fast_divides_delay_by_multiplier() {
        let timing = TimingModel::fast(2.0);
        assert_eq!(timing.delay_us(1000), 500);
        assert_eq!(timing.delay_us(2000), 1000);

        let timing = TimingModel::fast(10.0);
        assert_eq!(timing.delay_us(10_000), 1_000);
        assert_eq!(timing.delay_us(100_000), 10_000);
    }

    #[test]
    fn test_timing_fast_with_large_multiplier() {
        let timing = TimingModel::fast(50.0);
        assert_eq!(timing.delay_us(50_000), 1_000);
        assert_eq!(timing.delay_us(500_000), 10_000);
    }

    #[test]
    fn test_timing_default_is_realistic() {
        let timing = TimingModel::default();
        assert_eq!(timing.delay_us(1000), 1000);
    }

    #[test]
    fn test_timing_model_equality() {
        assert_eq!(TimingModel::none(), TimingModel::None);
        assert_eq!(TimingModel::realistic(), TimingModel::Realistic);
        assert_eq!(TimingModel::fast(2.0), TimingModel::Fast(2.0));
    }
}
