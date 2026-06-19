define_rpd_ome!(
    /// Non-singlet odd operator matrix element A_{qq,Q}^{NS,−}.
    ///
    /// This OME has regular, plus, and delta parts. It contributes to the
    /// flavour non-singlet combination with odd Mellin moments.
    ///
    /// # Example
    ///
    /// ```
    /// use libome_rs::AqqQNSOdd;
    ///
    /// let val = AqqQNSOdd::reg(0.25, -5.0, 3.0, 0.5);
    /// assert!(val.is_finite());
    /// ```
    AqqQNSOdd,
    AqqQNSOdd
);
