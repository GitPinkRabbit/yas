use anyhow::Result;
use log::info;
use yas::capture::{Capturer, GenericCapturer};
use yas::draw_capture_region::DrawCaptureRegion;
use yas::game_info::GameInfoBuilder;
use yas::window_info::{load_window_info_repo, FromWindowInfoRepository};

use yas_scanner_genshin::scanner::ArtifactScannerWindowInfo;

fn main() -> Result<()> {
    env_logger::init();

    // 1. 检测游戏窗口
    let game_info = GameInfoBuilder::new()
        .add_local_window_name("原神")
        .add_local_window_name("Genshin Impact")
        .add_cloud_window_name("云·原神")
        .build()?;

    info!("窗口位置: {:?}", game_info.window);
    info!("UI 类型: {:?}", game_info.ui);
    info!("分辨率族: {:?}", game_info.resolution_family);

    // 2. 截取整个游戏窗口
    let capturer = GenericCapturer::new()?;
    let mut screenshot = capturer.capture_rect(game_info.window)?;
    info!("截图尺寸: {}x{}", screenshot.width(), screenshot.height());

    // 3. 加载窗口信息坐标
    let repo = load_window_info_repo!(
        "../../window_info/windows1600x900.json",
        "../../window_info/windows1280x960.json",
        "../../window_info/windows1440x900.json",
        "../../window_info/windows2100x900.json",
        "../../window_info/windows3440x1440.json",
    );

    let window_info = ArtifactScannerWindowInfo::from_window_info_repository(
        game_info.window.to_rect_usize().size(),
        game_info.ui,
        game_info.platform,
        &repo,
    )?;

    // 4. 在截图上绘制各个捕获区域
    // 矩形区域 — 红色边框
    window_info.panel_rect.draw_capture_region(&mut screenshot);
    window_info.title_rect.draw_capture_region(&mut screenshot);
    window_info.main_stat_name_rect.draw_capture_region(&mut screenshot);
    window_info.main_stat_value_rect.draw_capture_region(&mut screenshot);
    window_info.sub_stat_1.draw_capture_region(&mut screenshot);
    window_info.sub_stat_2.draw_capture_region(&mut screenshot);
    window_info.sub_stat_3.draw_capture_region(&mut screenshot);
    window_info.sub_stat_4.draw_capture_region(&mut screenshot);
    window_info.level_rect.draw_capture_region(&mut screenshot);
    window_info.item_equip_rect.draw_capture_region(&mut screenshot);
    window_info.item_count_rect.draw_capture_region(&mut screenshot);

    // 点位 — 蓝色十字准线
    window_info.star_pos.draw_capture_region(&mut screenshot);
    window_info.scan_margin_pos.draw_capture_region(&mut screenshot);
    window_info.lock_pos.draw_capture_region(&mut screenshot);

    // 5. 输出结果图片
    let output_path = "debug_capture_regions.png";
    screenshot.save(output_path)?;
    info!("调试截图已保存至: {}", output_path);
    info!("红色边框 = 捕获区域 (Rect)，蓝色十字 = 采样点位 (Pos)");

    Ok(())
}
