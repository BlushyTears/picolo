use picolo::readimg::*;
use picolo::plot::*;
use picolo::listops::*;

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    #[should_panic]
    fn try_to_plot_uneven()
    {
        let c_setting = PlotSettings::default();
        let x = vec![0, 152];
        let y = vec![0, 152, 100];
        plot_tbl(&x, &y, &c_setting);
    }

    // Normal plot
    #[test]
    fn normal_plot() {
        let c_setting = PlotSettings::default();
        let x = vec![0, 152, 0, 500, 50, 169];
        let y = vec![0, 152, 100, 0, 50, 602];
        plot_tbl(&x, &y, &c_setting);
    }

    // Test if log scale works
    #[test]
    fn large_scale_plot() {
        let c_setting = PlotSettings::default();
        let x = vec![0, 151235, 0, 500, 50, 12529];
        let y = vec![0, 12152, 12521, 254120, 521550, 602];
        plot_tbl(&x, &y, &c_setting);
    }

    #[test]
    fn large_scale_plot_neg() {
        let c_setting = PlotSettings::default();
        let x = vec![0, -1512352, -0, -500, 50, -1612529];
        let y = vec![0, -12152, -1512521, -1254120, -521550, -602];
        plot_tbl(&x, &y, &c_setting);
    }

    #[test]
    fn get_largest_elem() {
        let x = vec![0, 1512352, 0, 500, 50, 1612529]; // All pos
        let y = vec![0, -12152, -1512521, -12541, -521550, -602]; // All neg
        let z = vec![0, -2529, 0, -500, 50, 712529]; // Mixed
        assert_eq!(find_largest_abs_elem(x), 1612529 +300);
        assert_eq!(find_largest_abs_elem(y), -1512521 +300);
        assert_eq!(find_largest_abs_elem(z), 712529 +300);
    }

    #[test]
    fn read_img() {
        load_picture("images/icon.png", 100);
        load_picture("plot.png", 50);
    }

}