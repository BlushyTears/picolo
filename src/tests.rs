use picolo::readimg::*;
use picolo::plot::plot_tbl;

// Tests to be added later

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

    #[test]
    fn normal_plot() {
        let x = vec![0, 152, 142, 500, 50, 169];
        let y = vec![0, 152, 100, 600, 50, 602];
        plot_tbl(&x, &y);
    }

    #[test]
    fn read_img() {
        load_picture("images/icon.png", 100);
        load_picture("plot.png", 100);
    }

}