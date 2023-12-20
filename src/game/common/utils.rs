pub fn clamp<T: std::cmp::PartialOrd>(input: T, min: T, max: T) -> T {
    if input < min {
        return min;
    }
    if input > max {
        return max;
    }
    return input;
}
