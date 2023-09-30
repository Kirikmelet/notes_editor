use crate::app::App;
use crate::widgets::square::{SquareWidget, SquareWidgetDesc};
use crate::widgets::{Widget, WidgetRender};

pub async fn run() -> anyhow::Result<()> {
    let sample_box = SquareWidget::new(SquareWidgetDesc {
        width: 100.0,
        height: 100.0,
        x: 0.0,
        y: 0.0,
        color: [1.0, 1.0, 1.0, 1.2],
    })
    .build();
    let sample_box1 = SquareWidget::new(SquareWidgetDesc {
        width: 100.0,
        height: 20.0,
        x: 00.0,
        y: 00.0,
        color: [1.0, 0.0, 0.0, 1.0],
    })
    .build();
    let vectored_widget: Vec<Box<dyn WidgetRender>> = vec![sample_box, sample_box1];
    App::new(vectored_widget).run().await?;
    Ok(())
}
