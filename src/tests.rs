use picolo::readimg::*;
use picolo::plot::plot_tbl;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn try_to_plot_uneven()
    {
        let x = vec![0, 152];
        let y = vec![0, 152, 100];
        plot_tbl(&x, &y);
    }

    // Normal plot
    #[test]
    fn normal_plot() {
        let x = vec![0, 152, 0, 500, 50, 169];
        let y = vec![0, 152, 100, 0, 50, 602];
        plot_tbl(&x, &y);
    }

    // Test if log scale works
    #[test]
    fn large_scale_plot() {
        let x = vec![0, 1512352, 0, 500, 50, 1612529];
        let y = vec![0, -12152, 1512521, 1254120, 521550, 602];
        plot_tbl(&x, &y);
    }

    #[test]
    fn read_img() {
        load_picture("images/icon.png", 100);
        load_picture("plot.png", 50);
    }

}