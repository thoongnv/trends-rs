use std::collections::HashMap;
use std::env;
use std::fmt::Write;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use unicode_width::UnicodeWidthStr;

use ratatui::backend::TestBackend;
use ratatui::prelude::*;
use ratatui::Terminal;

use strend::app::{App, AppResult, AppState};
use strend::components::Component;
use strend::handler::handle_events;
use strend::ui;
use strend::util;

#[test]
fn launch_app_and_make_few_searches() -> AppResult<()> {
    let mut server = mockito::Server::new();

    // Only mock tests if running in Github CI
    if let Ok(_) = env::var("CARGO_PKG_NAME") {
        // Create temp key file as we will mock API requests later
        if let Err(_) = util::get_api_key() {
            util::init_api_key("invalid".to_string(), false)?;
        }

        // Mock API url
        env::set_var("MOCK_API_URL", server.url());

        // Create mock requests for below tests
        server
            .mock("GET", "/api/v1/search")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("query".into(), "nginx".into()),
                mockito::Matcher::UrlEncoded("facets".into(), "os:5".into()),
            ]))
            .with_body(r#"{"total": 2291301747, "matches": [{"month": "2017-06", "count": 19799459}, {"month": "2017-07", "count": 21077099}, {"month": "2017-08", "count": 20541902}, {"month": "2017-09", "count": 18909404}, {"month": "2017-10", "count": 18645169}, {"month": "2017-11", "count": 21282285}, {"month": "2017-12", "count": 22234309}, {"month": "2018-01", "count": 20949579}, {"month": "2018-02", "count": 20605463}, {"month": "2018-03", "count": 16719816}, {"month": "2018-04", "count": 18607126}, {"month": "2018-05", "count": 18926021}, {"month": "2018-06", "count": 19928185}, {"month": "2018-07", "count": 19101790}, {"month": "2018-08", "count": 20660632}, {"month": "2018-09", "count": 21492383}, {"month": "2018-10", "count": 21660903}, {"month": "2018-11", "count": 16454098}, {"month": "2018-12", "count": 17100739}, {"month": "2019-01", "count": 17338056}, {"month": "2019-02", "count": 17587407}, {"month": "2019-03", "count": 18099830}, {"month": "2019-04", "count": 18694563}, {"month": "2019-05", "count": 19575447}, {"month": "2019-06", "count": 19339535}, {"month": "2019-07", "count": 20159173}, {"month": "2019-08", "count": 22084681}, {"month": "2019-09", "count": 25031109}, {"month": "2019-10", "count": 27390012}, {"month": "2019-11", "count": 30009448}, {"month": "2019-12", "count": 32878528}, {"month": "2020-01", "count": 34278063}, {"month": "2020-02", "count": 34248061}, {"month": "2020-03", "count": 35727267}, {"month": "2020-04", "count": 33514861}, {"month": "2020-05", "count": 31185488}, {"month": "2020-06", "count": 30585935}, {"month": "2020-07", "count": 32593481}, {"month": "2020-08", "count": 33444410}, {"month": "2020-09", "count": 32279025}, {"month": "2020-10", "count": 32610465}, {"month": "2020-11", "count": 35403858}, {"month": "2020-12", "count": 36644960}, {"month": "2021-01", "count": 34029229}, {"month": "2021-02", "count": 30687033}, {"month": "2021-03", "count": 32105288}, {"month": "2021-04", "count": 33825097}, {"month": "2021-05", "count": 34940276}, {"month": "2021-06", "count": 35969156}, {"month": "2021-07", "count": 39090908}, {"month": "2021-08", "count": 39496700}, {"month": "2021-09", "count": 41588700}, {"month": "2021-10", "count": 42880681}, {"month": "2021-11", "count": 41811780}, {"month": "2021-12", "count": 40707691}, {"month": "2022-01", "count": 40931338}, {"month": "2022-02", "count": 41699624}, {"month": "2022-03", "count": 45766022}, {"month": "2022-04", "count": 44967751}, {"month": "2022-05", "count": 44355027}, {"month": "2022-06", "count": 42024798}, {"month": "2022-07", "count": 40901754}, {"month": "2022-08", "count": 39843440}, {"month": "2022-09", "count": 38904796}, {"month": "2022-10", "count": 37806717}, {"month": "2022-11", "count": 38461775}, {"month": "2022-12", "count": 43186864}, {"month": "2023-01", "count": 42039035}, {"month": "2023-02", "count": 38207590}, {"month": "2023-03", "count": 40433229}, {"month": "2023-04", "count": 38729080}, {"month": "2023-05", "count": 40141267}, {"month": "2023-06", "count": 38299048}, {"month": "2023-07", "count": 38045728}, {"month": "2023-08", "count": 24024300}], "facets": {"os": [{"key": "2017-06", "values": [{"count": 164491, "value": "Linux 3.x"}, {"count": 14118, "value": "Linux 2.6.x"}, {"count": 3115, "value": "FreeBSD 9.x"}, {"count": 2943, "value": "Windows 7 or 8"}, {"count": 776, "value": "Linux 2.4-2.6"}]}, {"key": "2017-07", "values": [{"count": 171330, "value": "Linux 3.x"}, {"count": 11790, "value": "Linux 2.6.x"}, {"count": 3430, "value": "Windows 7 or 8"}, {"count": 2507, "value": "FreeBSD 9.x"}, {"count": 729, "value": "Linux 2.4-2.6"}]}, {"key": "2017-08", "values": [{"count": 162060, "value": "Linux 3.x"}, {"count": 11061, "value": "Linux 2.6.x"}, {"count": 3867, "value": "Windows 7 or 8"}, {"count": 2441, "value": "FreeBSD 9.x"}, {"count": 735, "value": "Linux 2.4-2.6"}]}, {"key": "2017-09", "values": [{"count": 166965, "value": "Linux 3.x"}, {"count": 11245, "value": "Linux 2.6.x"}, {"count": 4800, "value": "Windows 7 or 8"}, {"count": 3472, "value": "FreeBSD 9.x"}, {"count": 694, "value": "Linux 2.4-2.6"}]}, {"key": "2017-10", "values": [{"count": 167049, "value": "Linux 3.x"}, {"count": 9701, "value": "Linux 2.6.x"}, {"count": 4056, "value": "Windows 7 or 8"}, {"count": 3599, "value": "FreeBSD 9.x"}, {"count": 653, "value": "Linux 2.4-2.6"}]}, {"key": "2017-11", "values": [{"count": 179878, "value": "Linux 3.x"}, {"count": 8653, "value": "Linux 2.6.x"}, {"count": 8005, "value": "Windows 7 or 8"}, {"count": 3300, "value": "FreeBSD 9.x"}, {"count": 1632, "value": "Windows XP"}]}, {"key": "2017-12", "values": [{"count": 166923, "value": "Linux 3.x"}, {"count": 12248, "value": "Windows 7 or 8"}, {"count": 7972, "value": "Linux 2.6.x"}, {"count": 3219, "value": "FreeBSD 9.x"}, {"count": 2732, "value": "Windows XP"}]}, {"key": "2018-01", "values": [{"count": 157443, "value": "Linux 3.x"}, {"count": 12113, "value": "Windows 7 or 8"}, {"count": 7649, "value": "Linux 2.6.x"}, {"count": 3391, "value": "FreeBSD 9.x"}, {"count": 2877, "value": "Windows XP"}]}, {"key": "2018-02", "values": [{"count": 148588, "value": "Linux 3.x"}, {"count": 9236, "value": "Windows 7 or 8"}, {"count": 7242, "value": "Linux 2.6.x"}, {"count": 3112, "value": "FreeBSD 9.x"}, {"count": 1950, "value": "Windows XP"}]}, {"key": "2018-03", "values": [{"count": 141679, "value": "Linux 3.x"}, {"count": 8942, "value": "Windows 7 or 8"}, {"count": 6429, "value": "Linux 2.6.x"}, {"count": 2389, "value": "FreeBSD 9.x"}, {"count": 1795, "value": "Windows XP"}]}, {"key": "2018-04", "values": [{"count": 124158, "value": "Linux 3.x"}, {"count": 7676, "value": "Windows 7 or 8"}, {"count": 5640, "value": "Linux 2.6.x"}, {"count": 2229, "value": "FreeBSD 9.x"}, {"count": 1423, "value": "Windows XP"}]}, {"key": "2018-05", "values": [{"count": 162338, "value": "Linux 3.x"}, {"count": 7647, "value": "Windows 7 or 8"}, {"count": 7466, "value": "Linux 2.6.x"}, {"count": 2524, "value": "FreeBSD 9.x"}, {"count": 1138, "value": "Windows XP"}]}, {"key": "2018-06", "values": [{"count": 175500, "value": "Linux 3.x"}, {"count": 8049, "value": "Linux 2.6.x"}, {"count": 7776, "value": "Windows 7 or 8"}, {"count": 2766, "value": "FreeBSD 9.x"}, {"count": 943, "value": "Windows XP"}]}, {"key": "2018-07", "values": [{"count": 167129, "value": "Linux 3.x"}, {"count": 7235, "value": "Windows 7 or 8"}, {"count": 6915, "value": "Linux 2.6.x"}, {"count": 2365, "value": "FreeBSD 9.x"}, {"count": 846, "value": "Windows XP"}]}, {"key": "2018-08", "values": [{"count": 127048, "value": "Linux 3.x"}, {"count": 5831, "value": "Windows 7 or 8"}, {"count": 4717, "value": "Linux 2.6.x"}, {"count": 1378, "value": "FreeBSD 9.x"}, {"count": 731, "value": "Windows XP"}]}, {"key": "2018-09", "values": [{"count": 129785, "value": "Linux 3.x"}, {"count": 8144, "value": "Windows 7 or 8"}, {"count": 4093, "value": "Linux 2.6.x"}, {"count": 1174, "value": "Windows XP"}, {"count": 1102, "value": "FreeBSD 9.x"}]}, {"key": "2018-10", "values": [{"count": 179200, "value": "Linux 3.x"}, {"count": 13954, "value": "Windows 7 or 8"}, {"count": 6155, "value": "Linux 2.6.x"}, {"count": 2024, "value": "FreeBSD 9.x"}, {"count": 1846, "value": "Windows XP"}]}, {"key": "2018-11", "values": [{"count": 205294, "value": "Linux 3.x"}, {"count": 20103, "value": "Windows 7 or 8"}, {"count": 9326, "value": "Linux 2.6.x"}, {"count": 4441, "value": "FreeBSD 9.x"}, {"count": 1942, "value": "Windows XP"}]}, {"key": "2018-12", "values": [{"count": 242950, "value": "Linux 3.x"}, {"count": 30327, "value": "Windows 7 or 8"}, {"count": 10985, "value": "Linux 2.6.x"}, {"count": 5158, "value": "FreeBSD 9.x"}, {"count": 1932, "value": "Windows XP"}]}, {"key": "2019-01", "values": [{"count": 278370, "value": "Linux 3.x"}, {"count": 38568, "value": "Windows 7 or 8"}, {"count": 14293, "value": "Linux 2.6.x"}, {"count": 6656, "value": "FreeBSD 9.x"}, {"count": 2489, "value": "Windows XP"}]}, {"key": "2019-02", "values": [{"count": 210478, "value": "Linux 3.x"}, {"count": 31790, "value": "Windows 7 or 8"}, {"count": 10277, "value": "Linux 2.6.x"}, {"count": 4697, "value": "FreeBSD 9.x"}, {"count": 1898, "value": "Windows XP"}]}, {"key": "2019-03", "values": [{"count": 178362, "value": "Linux 3.x"}, {"count": 29785, "value": "Windows 7 or 8"}, {"count": 8857, "value": "Linux 2.6.x"}, {"count": 3723, "value": "FreeBSD 9.x"}, {"count": 1652, "value": "Windows XP"}]}, {"key": "2019-04", "values": [{"count": 192176, "value": "Linux 3.x"}, {"count": 43104, "value": "Windows 7 or 8"}, {"count": 8450, "value": "Linux 2.6.x"}, {"count": 3850, "value": "FreeBSD 9.x"}, {"count": 1831, "value": "Windows XP"}]}, {"key": "2019-05", "values": [{"count": 196790, "value": "Linux 3.x"}, {"count": 51132, "value": "Windows 7 or 8"}, {"count": 8711, "value": "Linux 2.6.x"}, {"count": 3829, "value": "FreeBSD 9.x"}, {"count": 2042, "value": "Windows XP"}]}, {"key": "2019-06", "values": [{"count": 183186, "value": "Linux 3.x"}, {"count": 59451, "value": "Windows 7 or 8"}, {"count": 7820, "value": "Linux 2.6.x"}, {"count": 3026, "value": "FreeBSD 9.x"}, {"count": 1828, "value": "Windows XP"}]}, {"key": "2019-07", "values": [{"count": 186114, "value": "Linux 3.x"}, {"count": 69115, "value": "Windows 7 or 8"}, {"count": 7551, "value": "Linux 2.6.x"}, {"count": 3600, "value": "FreeBSD 9.x"}, {"count": 1580, "value": "Windows XP"}]}, {"key": "2019-08", "values": [{"count": 204648, "value": "Linux 3.x"}, {"count": 85578, "value": "Windows 7 or 8"}, {"count": 9111, "value": "Linux 2.6.x"}, {"count": 3970, "value": "FreeBSD 9.x"}, {"count": 1758, "value": "Windows XP"}]}, {"key": "2019-09", "values": [{"count": 184587, "value": "Linux 3.x"}, {"count": 80807, "value": "Windows 7 or 8"}, {"count": 7453, "value": "Linux 2.6.x"}, {"count": 3406, "value": "FreeBSD 9.x"}, {"count": 1567, "value": "Windows XP"}]}, {"key": "2019-10", "values": [{"count": 214056, "value": "Linux 3.x"}, {"count": 98799, "value": "Windows 7 or 8"}, {"count": 6622, "value": "Linux 2.6.x"}, {"count": 3854, "value": "FreeBSD 9.x"}, {"count": 1844, "value": "Windows XP"}]}, {"key": "2019-11", "values": [{"count": 237675, "value": "Linux 3.x"}, {"count": 116240, "value": "Windows 7 or 8"}, {"count": 6935, "value": "Linux 2.6.x"}, {"count": 5527, "value": "FreeBSD 9.x"}, {"count": 2344, "value": "Windows XP"}]}, {"key": "2019-12", "values": [{"count": 241232, "value": "Linux 3.x"}, {"count": 122432, "value": "Windows 7 or 8"}, {"count": 7079, "value": "Linux 2.6.x"}, {"count": 4517, "value": "FreeBSD 9.x"}, {"count": 3021, "value": "Windows XP"}]}, {"key": "2020-01", "values": [{"count": 229448, "value": "Linux 3.x"}, {"count": 102544, "value": "Windows Server 2008"}, {"count": 15851, "value": "Windows 7 or 8"}, {"count": 6942, "value": "Linux 2.6.x"}, {"count": 4000, "value": "FreeBSD 9.x"}]}, {"key": "2020-02", "values": [{"count": 212965, "value": "Linux 3.x"}, {"count": 107404, "value": "Windows Server 2008"}, {"count": 8776, "value": "Windows 7 or 8"}, {"count": 6735, "value": "Linux 2.6.x"}, {"count": 4048, "value": "FreeBSD 9.x"}]}, {"key": "2020-03", "values": [{"count": 197589, "value": "Linux 3.x"}, {"count": 93457, "value": "Windows Server 2008"}, {"count": 11987, "value": "Windows 7 or 8"}, {"count": 6098, "value": "Linux 2.6.x"}, {"count": 4254, "value": "FreeBSD 9.x"}]}, {"key": "2020-04", "values": [{"count": 133853, "value": "Linux 3.x"}, {"count": 74913, "value": "Windows Server 2008"}, {"count": 5769, "value": "Windows 7 or 8"}, {"count": 4121, "value": "Linux 2.6.x"}, {"count": 3229, "value": "FreeBSD 9.x"}]}, {"key": "2020-05", "values": [{"count": 78271, "value": "Linux 3.x"}, {"count": 44444, "value": "Windows Server 2008"}, {"count": 3294, "value": "Linux 2.6.x"}, {"count": 3222, "value": "FreeBSD 9.x"}, {"count": 761, "value": "Windows 7 or 8"}]}, {"key": "2020-06", "values": [{"count": 60571, "value": "Linux 3.x"}, {"count": 32597, "value": "Windows Server 2008"}, {"count": 2874, "value": "Linux 2.6.x"}, {"count": 2557, "value": "FreeBSD 9.x"}, {"count": 753, "value": "Windows 7 or 8"}]}, {"key": "2020-07", "values": [{"count": 37818, "value": "Linux 3.x"}, {"count": 22423, "value": "Windows Server 2008"}, {"count": 2057, "value": "Linux 2.6.x"}, {"count": 1917, "value": "FreeBSD 9.x"}, {"count": 899, "value": "Windows 7 or 8"}]}, {"key": "2020-08", "values": [{"count": 9724, "value": "Linux 3.x"}, {"count": 1168, "value": "Windows 7 or 8"}, {"count": 1118, "value": "Linux 2.6.x"}, {"count": 682, "value": "Windows Server 2008"}, {"count": 529, "value": "FreeBSD 9.x"}]}, {"key": "2020-09", "values": [{"count": 1151, "value": "Linux 3.x"}, {"count": 488, "value": "Windows 7 or 8"}, {"count": 178, "value": "FreeBSD 9.x"}, {"count": 140, "value": "Windows Server 2008"}, {"count": 105, "value": "Linux 2.6.x"}]}, {"key": "2020-10", "values": [{"count": 33, "value": "linux"}, {"count": 15, "value": "Windows 6.1"}, {"count": 7, "value": "Unix"}, {"count": 1, "value": "PAN-OS 9.1.4"}]}, {"key": "2020-11", "values": [{"count": 49, "value": "linux"}, {"count": 14, "value": "Windows 6.1"}, {"count": 8, "value": "Unix"}, {"count": 1, "value": "PAN-OS 9.1.4"}]}, {"key": "2020-12", "values": [{"count": 53, "value": "linux"}, {"count": 12, "value": "Windows 6.1"}, {"count": 6, "value": "Unix"}, {"count": 1, "value": "PAN-OS 9.1.4"}]}, {"key": "2021-01", "values": [{"count": 48, "value": "linux"}, {"count": 11, "value": "Windows 6.1"}, {"count": 8, "value": "Unix"}]}, {"key": "2021-02", "values": [{"count": 32, "value": "linux"}, {"count": 11, "value": "Windows 6.1"}, {"count": 8, "value": "Unix"}, {"count": 1, "value": "PAN-OS 9.1.4"}, {"count": 1, "value": "QTS"}]}, {"key": "2021-03", "values": [{"count": 27, "value": "linux"}, {"count": 9, "value": "Unix"}, {"count": 9, "value": "Windows 6.1"}, {"count": 1, "value": "PAN-OS 9.1.4"}]}, {"key": "2021-04", "values": [{"count": 3913, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 2121, "value": "Synology DiskStation Manager (DSM) 7.1.1-42962"}, {"count": 1324, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 854, "value": "Synology DiskStation Manager (DSM) 7.1-42661"}, {"count": 832, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}]}, {"key": "2021-05", "values": [{"count": 15, "value": "linux"}, {"count": 9, "value": "Unix"}, {"count": 9, "value": "Windows 6.1"}, {"count": 1, "value": "PAN-OS 9.1.4"}]}, {"key": "2021-06", "values": [{"count": 544148, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 154926, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}, {"count": 59578, "value": "Synology DiskStation Manager (DSM) 6.2.2-24922"}, {"count": 29594, "value": "Synology DiskStation Manager (DSM) 6.1.7-15284"}, {"count": 20643, "value": "Synology DiskStation Manager (DSM) 6.2.1-23824"}]}, {"key": "2021-07", "values": [{"count": 620262, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 168567, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}, {"count": 67439, "value": "Synology DiskStation Manager (DSM) 6.2.2-24922"}, {"count": 66522, "value": "Synology DiskStation Manager (DSM) 7.0-41890"}, {"count": 36288, "value": "Synology DiskStation Manager (DSM) 6.1.7-15284"}]}, {"key": "2021-08", "values": [{"count": 521920, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 166671, "value": "Synology DiskStation Manager (DSM) 7.0-41890"}, {"count": 146996, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}, {"count": 61587, "value": "Synology DiskStation Manager (DSM) 6.2.2-24922"}, {"count": 35142, "value": "Synology DiskStation Manager (DSM) 6.1.7-15284"}]}, {"key": "2021-09", "values": [{"count": 474727, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 219610, "value": "Synology DiskStation Manager (DSM) 7.0-41890"}, {"count": 137921, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}, {"count": 59687, "value": "Synology DiskStation Manager (DSM) 6.2.2-24922"}, {"count": 34392, "value": "Synology DiskStation Manager (DSM) 6.1.7-15284"}]}, {"key": "2021-10", "values": [{"count": 458185, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 229925, "value": "Synology DiskStation Manager (DSM) 7.0-41890"}, {"count": 129853, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}, {"count": 60707, "value": "Synology DiskStation Manager (DSM)"}, {"count": 58212, "value": "Synology DiskStation Manager (DSM) 6.2.2-24922"}]}, {"key": "2021-11", "values": [{"count": 421577, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 164777, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 126173, "value": "Synology DiskStation Manager (DSM) 7.0-41890"}, {"count": 117698, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}, {"count": 52122, "value": "Synology DiskStation Manager (DSM) 6.2.2-24922"}]}, {"key": "2021-12", "values": [{"count": 403216, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 235039, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 109338, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}, {"count": 84251, "value": "Synology DiskStation Manager (DSM) 7.0-41890"}, {"count": 47745, "value": "Synology DiskStation Manager (DSM) 6.2.2-24922"}]}, {"key": "2022-01", "values": [{"count": 399086, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 283525, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 105386, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}, {"count": 66893, "value": "Synology DiskStation Manager (DSM) 7.0-41890"}, {"count": 47441, "value": "Synology DiskStation Manager (DSM) 6.2.2-24922"}]}, {"key": "2022-02", "values": [{"count": 365198, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 297552, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 93800, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}, {"count": 51960, "value": "Synology DiskStation Manager (DSM) 7.0-41890"}, {"count": 42966, "value": "Synology DiskStation Manager (DSM) 6.2.2-24922"}]}, {"key": "2022-03", "values": [{"count": 384155, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 357803, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 99166, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}, {"count": 50066, "value": "Synology DiskStation Manager (DSM) 7.0-41890"}, {"count": 45396, "value": "Synology DiskStation Manager (DSM) 6.2.2-24922"}]}, {"key": "2022-04", "values": [{"count": 364076, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 340651, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 92094, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}, {"count": 43010, "value": "Synology DiskStation Manager (DSM) 6.2.2-24922"}, {"count": 42839, "value": "Synology DiskStation Manager (DSM) 7.0-41890"}]}, {"key": "2022-05", "values": [{"count": 343930, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 286944, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 87647, "value": "Synology DiskStation Manager (DSM) 7.1-42661"}, {"count": 84499, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}, {"count": 40148, "value": "Synology DiskStation Manager (DSM) 6.2.2-24922"}]}, {"key": "2022-06", "values": [{"count": 354196, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 213881, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 202171, "value": "Synology DiskStation Manager (DSM) 7.1-42661"}, {"count": 87323, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}, {"count": 40857, "value": "Synology DiskStation Manager (DSM) 6.2.2-24922"}]}, {"key": "2022-07", "values": [{"count": 363919, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 284055, "value": "Synology DiskStation Manager (DSM) 7.1-42661"}, {"count": 169438, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 88680, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}, {"count": 40667, "value": "Synology DiskStation Manager (DSM) 6.2.2-24922"}]}, {"key": "2022-08", "values": [{"count": 341170, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 304322, "value": "Synology DiskStation Manager (DSM) 7.1-42661"}, {"count": 141653, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 82502, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}, {"count": 38240, "value": "Synology DiskStation Manager (DSM) 6.2.2-24922"}]}, {"key": "2022-09", "values": [{"count": 289794, "value": "Synology DiskStation Manager (DSM) 7.1-42661"}, {"count": 279405, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 119373, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 66017, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}, {"count": 37286, "value": "Synology DiskStation Manager (DSM)"}]}, {"key": "2022-10", "values": [{"count": 300060, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 184421, "value": "Synology DiskStation Manager (DSM) 7.1-42661"}, {"count": 133176, "value": "Synology DiskStation Manager (DSM) 7.1.1-42962"}, {"count": 105176, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 71541, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}]}, {"key": "2022-11", "values": [{"count": 297273, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 247675, "value": "Synology DiskStation Manager (DSM) 7.1.1-42962"}, {"count": 116829, "value": "Synology DiskStation Manager (DSM) 7.1-42661"}, {"count": 96379, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 70897, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}]}, {"key": "2022-12", "values": [{"count": 2612496, "value": "Linux"}, {"count": 305469, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 303062, "value": "Synology DiskStation Manager (DSM) 7.1.1-42962"}, {"count": 100908, "value": "Synology DiskStation Manager (DSM) 7.1-42661"}, {"count": 97753, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}]}, {"key": "2023-01", "values": [{"count": 2678421, "value": "Linux"}, {"count": 312167, "value": "Synology DiskStation Manager (DSM) 7.1.1-42962"}, {"count": 284498, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 83940, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 76950, "value": "Synology DiskStation Manager (DSM) 7.1-42661"}]}, {"key": "2023-02", "values": [{"count": 2642719, "value": "Linux"}, {"count": 285554, "value": "Synology DiskStation Manager (DSM) 7.1.1-42962"}, {"count": 242001, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 66012, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 56945, "value": "Synology DiskStation Manager (DSM) 7.1-42661"}]}, {"key": "2023-03", "values": [{"count": 2690022, "value": "Linux"}, {"count": 350177, "value": "Synology DiskStation Manager (DSM) 7.1.1-42962"}, {"count": 260502, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 70288, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 58758, "value": "Synology DiskStation Manager (DSM) 7.1-42661"}]}, {"key": "2023-04", "values": [{"count": 2680674, "value": "Linux"}, {"count": 327911, "value": "Synology DiskStation Manager (DSM) 7.1.1-42962"}, {"count": 232739, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 59310, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}, {"count": 51758, "value": "Synology DiskStation Manager (DSM) 6.2.3-25426"}]}, {"key": "2023-05", "values": [{"count": 2755093, "value": "Ubuntu"}, {"count": 379661, "value": "Synology DiskStation Manager (DSM) 7.1.1-42962"}, {"count": 245414, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 236408, "value": "Linux"}, {"count": 61456, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}]}, {"key": "2023-06", "values": [{"count": 2787611, "value": "Ubuntu"}, {"count": 327996, "value": "Synology DiskStation Manager (DSM) 7.1.1-42962"}, {"count": 214599, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 137773, "value": "Linux"}, {"count": 52828, "value": "Synology DiskStation Manager (DSM) 7.0.1-42218"}]}, {"key": "2023-07", "values": [{"count": 1991012, "value": "Ubuntu"}, {"count": 225292, "value": "Synology DiskStation Manager (DSM) 7.1.1-42962"}, {"count": 166477, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 136934, "value": "Linux"}, {"count": 65760, "value": "Synology DiskStation Manager (DSM) 7.2-64570"}]}, {"key": "2023-08", "values": [{"count": 121778, "value": "Ubuntu"}, {"count": 102540, "value": "Linux"}, {"count": 12129, "value": "Synology DiskStation Manager (DSM) 7.1.1-42962"}, {"count": 11212, "value": "Synology DiskStation Manager (DSM) 6.2.4-25556"}, {"count": 6540, "value": "Synology DiskStation Manager (DSM)"}]}]}}"#)
            .create();

        server
            .mock("GET", "/api/v1/search")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("query".into(), "port:111222".into()),
                mockito::Matcher::UrlEncoded("facets".into(), "org".into()),
            ]))
            .with_body(r#"{"total": 0, "matches": [{"month": "2017-06", "count": 0}, {"month": "2017-07", "count": 0}, {"month": "2017-08", "count": 0}, {"month": "2017-09", "count": 0}, {"month": "2017-10", "count": 0}, {"month": "2017-11", "count": 0}, {"month": "2017-12", "count": 0}, {"month": "2018-01", "count": 0}, {"month": "2018-02", "count": 0}, {"month": "2018-03", "count": 0}, {"month": "2018-04", "count": 0}, {"month": "2018-05", "count": 0}, {"month": "2018-06", "count": 0}, {"month": "2018-07", "count": 0}, {"month": "2018-08", "count": 0}, {"month": "2018-09", "count": 0}, {"month": "2018-10", "count": 0}, {"month": "2018-11", "count": 0}, {"month": "2018-12", "count": 0}, {"month": "2019-01", "count": 0}, {"month": "2019-02", "count": 0}, {"month": "2019-03", "count": 0}, {"month": "2019-04", "count": 0}, {"month": "2019-05", "count": 0}, {"month": "2019-06", "count": 0}, {"month": "2019-07", "count": 0}, {"month": "2019-08", "count": 0}, {"month": "2019-09", "count": 0}, {"month": "2019-10", "count": 0}, {"month": "2019-11", "count": 0}, {"month": "2019-12", "count": 0}, {"month": "2020-01", "count": 0}, {"month": "2020-02", "count": 0}, {"month": "2020-03", "count": 0}, {"month": "2020-04", "count": 0}, {"month": "2020-05", "count": 0}, {"month": "2020-06", "count": 0}, {"month": "2020-07", "count": 0}, {"month": "2020-08", "count": 0}, {"month": "2020-09", "count": 0}, {"month": "2020-10", "count": 0}, {"month": "2020-11", "count": 0}, {"month": "2020-12", "count": 0}, {"month": "2021-01", "count": 0}, {"month": "2021-02", "count": 0}, {"month": "2021-03", "count": 0}, {"month": "2021-04", "count": 0}, {"month": "2021-05", "count": 0}, {"month": "2021-06", "count": 0}, {"month": "2021-07", "count": 0}, {"month": "2021-08", "count": 0}, {"month": "2021-09", "count": 0}, {"month": "2021-10", "count": 0}, {"month": "2021-11", "count": 0}, {"month": "2021-12", "count": 0}, {"month": "2022-01", "count": 0}, {"month": "2022-02", "count": 0}, {"month": "2022-03", "count": 0}, {"month": "2022-04", "count": 0}, {"month": "2022-05", "count": 0}, {"month": "2022-06", "count": 0}, {"month": "2022-07", "count": 0}, {"month": "2022-08", "count": 0}, {"month": "2022-09", "count": 0}, {"month": "2022-10", "count": 0}, {"month": "2022-11", "count": 0}, {"month": "2022-12", "count": 0}, {"month": "2023-01", "count": 0}, {"month": "2023-02", "count": 0}, {"month": "2023-03", "count": 0}, {"month": "2023-04", "count": 0}, {"month": "2023-05", "count": 0}, {"month": "2023-06", "count": 0}, {"month": "2023-07", "count": 0}, {"month": "2023-08", "count": 0}], "facets": {"org": [{"key": "2017-06", "values": []}, {"key": "2017-07", "values": []}, {"key": "2017-08", "values": []}, {"key": "2017-09", "values": []}, {"key": "2017-10", "values": []}, {"key": "2017-11", "values": []}, {"key": "2017-12", "values": []}, {"key": "2018-01", "values": []}, {"key": "2018-02", "values": []}, {"key": "2018-03", "values": []}, {"key": "2018-04", "values": []}, {"key": "2018-05", "values": []}, {"key": "2018-06", "values": []}, {"key": "2018-07", "values": []}, {"key": "2018-08", "values": []}, {"key": "2018-09", "values": []}, {"key": "2018-10", "values": []}, {"key": "2018-11", "values": []}, {"key": "2018-12", "values": []}, {"key": "2019-01", "values": []}, {"key": "2019-02", "values": []}, {"key": "2019-03", "values": []}, {"key": "2019-04", "values": []}, {"key": "2019-05", "values": []}, {"key": "2019-06", "values": []}, {"key": "2019-07", "values": []}, {"key": "2019-08", "values": []}, {"key": "2019-09", "values": []}, {"key": "2019-10", "values": []}, {"key": "2019-11", "values": []}, {"key": "2019-12", "values": []}, {"key": "2020-01", "values": []}, {"key": "2020-02", "values": []}, {"key": "2020-03", "values": []}, {"key": "2020-04", "values": []}, {"key": "2020-05", "values": []}, {"key": "2020-06", "values": []}, {"key": "2020-07", "values": []}, {"key": "2020-08", "values": []}, {"key": "2020-09", "values": []}, {"key": "2020-10", "values": []}, {"key": "2020-11", "values": []}, {"key": "2020-12", "values": []}, {"key": "2021-01", "values": []}, {"key": "2021-02", "values": []}, {"key": "2021-03", "values": []}, {"key": "2021-04", "values": []}, {"key": "2021-05", "values": []}, {"key": "2021-06", "values": []}, {"key": "2021-07", "values": []}, {"key": "2021-08", "values": []}, {"key": "2021-09", "values": []}, {"key": "2021-10", "values": []}, {"key": "2021-11", "values": []}, {"key": "2021-12", "values": []}, {"key": "2022-01", "values": []}, {"key": "2022-02", "values": []}, {"key": "2022-03", "values": []}, {"key": "2022-04", "values": []}, {"key": "2022-05", "values": []}, {"key": "2022-06", "values": []}, {"key": "2022-07", "values": []}, {"key": "2022-08", "values": []}, {"key": "2022-09", "values": []}, {"key": "2022-10", "values": []}, {"key": "2022-11", "values": []}, {"key": "2022-12", "values": []}, {"key": "2023-01", "values": []}, {"key": "2023-02", "values": []}, {"key": "2023-03", "values": []}, {"key": "2023-04", "values": []}, {"key": "2023-05", "values": []}, {"key": "2023-06", "values": []}, {"key": "2023-07", "values": []}, {"key": "2023-08", "values": []}]}}"#)
            .create();

        server
            .mock("GET", "/api/v1/search")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("query".into(), "port:111222".into()),
                mockito::Matcher::UrlEncoded("facets".into(), "orggg".into()),
            ]))
            .with_status(400)
            .with_body(r#"{"error": "Invalid search facet"}"#)
            .create();

        server
            .mock("GET", "/api/v1/search")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("query".into(), "apache port:80".into()),
                mockito::Matcher::UrlEncoded("facets".into(), "org".into()),
            ]))
            .with_body(r#"{"total": 887190443, "matches": [{"month": "2017-06", "count": 12598695}, {"month": "2017-07", "count": 13318585}, {"month": "2017-08", "count": 13213818}, {"month": "2017-09", "count": 12740879}, {"month": "2017-10", "count": 13179828}, {"month": "2017-11", "count": 13231176}, {"month": "2017-12", "count": 15346154}, {"month": "2018-01", "count": 16264264}, {"month": "2018-02", "count": 12866803}, {"month": "2018-03", "count": 12881603}, {"month": "2018-04", "count": 13244892}, {"month": "2018-05", "count": 13425484}, {"month": "2018-06", "count": 13211595}, {"month": "2018-07", "count": 13249312}, {"month": "2018-08", "count": 13424098}, {"month": "2018-09", "count": 13250910}, {"month": "2018-10", "count": 13381559}, {"month": "2018-11", "count": 13391821}, {"month": "2018-12", "count": 13121292}, {"month": "2019-01", "count": 13129383}, {"month": "2019-02", "count": 13314743}, {"month": "2019-03", "count": 13072379}, {"month": "2019-04", "count": 12809859}, {"month": "2019-05", "count": 12735076}, {"month": "2019-06", "count": 12630357}, {"month": "2019-07", "count": 12449898}, {"month": "2019-08", "count": 12117927}, {"month": "2019-09", "count": 11823914}, {"month": "2019-10", "count": 11704150}, {"month": "2019-11", "count": 11751505}, {"month": "2019-12", "count": 11735424}, {"month": "2020-01", "count": 11551551}, {"month": "2020-02", "count": 11553281}, {"month": "2020-03", "count": 11926572}, {"month": "2020-04", "count": 11834486}, {"month": "2020-05", "count": 11530814}, {"month": "2020-06", "count": 11144103}, {"month": "2020-07", "count": 11575984}, {"month": "2020-08", "count": 11892773}, {"month": "2020-09", "count": 11920728}, {"month": "2020-10", "count": 11896146}, {"month": "2020-11", "count": 11748519}, {"month": "2020-12", "count": 11911130}, {"month": "2021-01", "count": 12011051}, {"month": "2021-02", "count": 11805293}, {"month": "2021-03", "count": 11996126}, {"month": "2021-04", "count": 11927950}, {"month": "2021-05", "count": 11747333}, {"month": "2021-06", "count": 11611992}, {"month": "2021-07", "count": 11514733}, {"month": "2021-08", "count": 11356047}, {"month": "2021-09", "count": 11511147}, {"month": "2021-10", "count": 11447801}, {"month": "2021-11", "count": 11479736}, {"month": "2021-12", "count": 11555765}, {"month": "2022-01", "count": 11533776}, {"month": "2022-02", "count": 11392814}, {"month": "2022-03", "count": 11235582}, {"month": "2022-04", "count": 11099026}, {"month": "2022-05", "count": 11021256}, {"month": "2022-06", "count": 10738456}, {"month": "2022-07", "count": 10988424}, {"month": "2022-08", "count": 10850693}, {"month": "2022-09", "count": 10865672}, {"month": "2022-10", "count": 10550802}, {"month": "2022-11", "count": 10598503}, {"month": "2022-12", "count": 10216609}, {"month": "2023-01", "count": 9985526}, {"month": "2023-02", "count": 9732912}, {"month": "2023-03", "count": 9706112}, {"month": "2023-04", "count": 9516838}, {"month": "2023-05", "count": 9388879}, {"month": "2023-06", "count": 9005797}, {"month": "2023-07", "count": 8899601}, {"month": "2023-08", "count": 4794721}], "facets": {"org": [{"key": "2017-06", "values": [{"count": 787842, "value": "Amazon.com"}, {"count": 271885, "value": "OVH SAS"}, {"count": 261872, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 260466, "value": "GoDaddy.com, LLC"}, {"count": 248595, "value": "MacroLAN"}]}, {"key": "2017-07", "values": [{"count": 828292, "value": "Amazon.com"}, {"count": 302148, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 282931, "value": "OVH SAS"}, {"count": 274392, "value": "GoDaddy.com, LLC"}, {"count": 256268, "value": "MacroLAN"}]}, {"key": "2017-08", "values": [{"count": 829150, "value": "Amazon.com"}, {"count": 315778, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 280428, "value": "OVH SAS"}, {"count": 273745, "value": "GoDaddy.com, LLC"}, {"count": 252201, "value": "MacroLAN"}]}, {"key": "2017-09", "values": [{"count": 763842, "value": "Amazon.com"}, {"count": 488157, "value": "New Guoxin Telecom Corporation"}, {"count": 340760, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 265934, "value": "GoDaddy.com, LLC"}, {"count": 264269, "value": "OVH SAS"}]}, {"key": "2017-10", "values": [{"count": 801203, "value": "Amazon.com"}, {"count": 350296, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 305925, "value": "New Guoxin Telecom Corporation"}, {"count": 286088, "value": "OVH SAS"}, {"count": 277274, "value": "GoDaddy.com, LLC"}]}, {"key": "2017-11", "values": [{"count": 821548, "value": "Amazon.com"}, {"count": 361400, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 303666, "value": "North Star Information Hi.tech Ltd. Co."}, {"count": 298092, "value": "OVH SAS"}, {"count": 286712, "value": "GoDaddy.com, LLC"}]}, {"key": "2017-12", "values": [{"count": 1108430, "value": "North Star Information Hi.tech Ltd. Co."}, {"count": 831368, "value": "Amazon.com"}, {"count": 380220, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 298751, "value": "OVH SAS"}, {"count": 291305, "value": "Korea Telecom"}]}, {"key": "2018-01", "values": [{"count": 1180975, "value": "North Star Information Hi.tech Ltd. Co."}, {"count": 831482, "value": "Amazon.com"}, {"count": 390293, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 304827, "value": "OVH SAS"}, {"count": 286863, "value": "GoDaddy.com, LLC"}]}, {"key": "2018-02", "values": [{"count": 845113, "value": "Amazon.com"}, {"count": 347548, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 307140, "value": "OVH SAS"}, {"count": 289627, "value": "GoDaddy.com, LLC"}, {"count": 201861, "value": "Digital Ocean"}]}, {"key": "2018-03", "values": [{"count": 856296, "value": "Amazon.com"}, {"count": 331096, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 305170, "value": "OVH SAS"}, {"count": 290631, "value": "GoDaddy.com, LLC"}, {"count": 199446, "value": "Digital Ocean"}]}, {"key": "2018-04", "values": [{"count": 879314, "value": "Amazon.com"}, {"count": 382299, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 295990, "value": "GoDaddy.com, LLC"}, {"count": 294390, "value": "OVH SAS"}, {"count": 217824, "value": "Digital Ocean"}]}, {"key": "2018-05", "values": [{"count": 902482, "value": "Amazon.com"}, {"count": 409518, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 302973, "value": "GoDaddy.com, LLC"}, {"count": 300794, "value": "OVH SAS"}, {"count": 221789, "value": "Digital Ocean"}]}, {"key": "2018-06", "values": [{"count": 891599, "value": "Amazon.com"}, {"count": 411051, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 303594, "value": "GoDaddy.com, LLC"}, {"count": 301207, "value": "OVH SAS"}, {"count": 217907, "value": "Digital Ocean"}]}, {"key": "2018-07", "values": [{"count": 904706, "value": "Amazon.com"}, {"count": 395235, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 305469, "value": "GoDaddy.com, LLC"}, {"count": 300742, "value": "OVH SAS"}, {"count": 216735, "value": "Digital Ocean"}]}, {"key": "2018-08", "values": [{"count": 941610, "value": "Amazon.com"}, {"count": 412928, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 307557, "value": "GoDaddy.com, LLC"}, {"count": 300204, "value": "OVH SAS"}, {"count": 212650, "value": "Digital Ocean"}]}, {"key": "2018-09", "values": [{"count": 887097, "value": "Amazon.com"}, {"count": 415105, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 304899, "value": "GoDaddy.com, LLC"}, {"count": 288940, "value": "OVH SAS"}, {"count": 236842, "value": "Digital Ocean"}]}, {"key": "2018-10", "values": [{"count": 902304, "value": "Amazon.com"}, {"count": 423981, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 312733, "value": "GoDaddy.com, LLC"}, {"count": 298675, "value": "OVH SAS"}, {"count": 243888, "value": "Digital Ocean"}]}, {"key": "2018-11", "values": [{"count": 903459, "value": "Amazon.com"}, {"count": 427740, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 299402, "value": "OVH SAS"}, {"count": 290836, "value": "GoDaddy.com, LLC"}, {"count": 241763, "value": "Digital Ocean"}]}, {"key": "2018-12", "values": [{"count": 921048, "value": "Amazon.com"}, {"count": 435302, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 302291, "value": "OVH SAS"}, {"count": 298387, "value": "GoDaddy.com, LLC"}, {"count": 246187, "value": "Digital Ocean"}]}, {"key": "2019-01", "values": [{"count": 929387, "value": "Amazon.com"}, {"count": 434784, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 309569, "value": "GoDaddy.com, LLC"}, {"count": 300984, "value": "OVH SAS"}, {"count": 241652, "value": "Digital Ocean"}]}, {"key": "2019-02", "values": [{"count": 922079, "value": "Amazon.com"}, {"count": 432166, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 321524, "value": "GoDaddy.com, LLC"}, {"count": 305888, "value": "OVH SAS"}, {"count": 240641, "value": "Digital Ocean"}]}, {"key": "2019-03", "values": [{"count": 934521, "value": "Amazon.com"}, {"count": 440600, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 322707, "value": "GoDaddy.com, LLC"}, {"count": 303703, "value": "OVH SAS"}, {"count": 238883, "value": "Digital Ocean"}]}, {"key": "2019-04", "values": [{"count": 919636, "value": "Amazon.com"}, {"count": 436727, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 321294, "value": "GoDaddy.com, LLC"}, {"count": 303097, "value": "OVH SAS"}, {"count": 235636, "value": "Digital Ocean"}]}, {"key": "2019-05", "values": [{"count": 930814, "value": "Amazon.com"}, {"count": 434217, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 323994, "value": "GoDaddy.com, LLC"}, {"count": 303134, "value": "OVH SAS"}, {"count": 232097, "value": "Digital Ocean"}]}, {"key": "2019-06", "values": [{"count": 884486, "value": "Amazon.com"}, {"count": 419039, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 319253, "value": "GoDaddy.com, LLC"}, {"count": 299554, "value": "OVH SAS"}, {"count": 239992, "value": "CloudInnovation infrastructure"}]}, {"key": "2019-07", "values": [{"count": 887917, "value": "Amazon.com"}, {"count": 416007, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 294042, "value": "GoDaddy.com, LLC"}, {"count": 292270, "value": "OVH SAS"}, {"count": 221413, "value": "Digital Ocean"}]}, {"key": "2019-08", "values": [{"count": 920838, "value": "Amazon.com"}, {"count": 404160, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 298126, "value": "GoDaddy.com, LLC"}, {"count": 271017, "value": "OVH SAS"}, {"count": 267840, "value": "Digital Ocean"}]}, {"key": "2019-09", "values": [{"count": 902981, "value": "Amazon.com"}, {"count": 393491, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 296856, "value": "GoDaddy.com, LLC"}, {"count": 269555, "value": "OVH SAS"}, {"count": 260838, "value": "Digital Ocean"}]}, {"key": "2019-10", "values": [{"count": 923751, "value": "Amazon.com"}, {"count": 396017, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 297556, "value": "GoDaddy.com, LLC"}, {"count": 271201, "value": "OVH SAS"}, {"count": 261728, "value": "Digital Ocean"}]}, {"key": "2019-11", "values": [{"count": 927467, "value": "Amazon.com"}, {"count": 391435, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 296818, "value": "GoDaddy.com, LLC"}, {"count": 275673, "value": "OVH SAS"}, {"count": 265393, "value": "Digital Ocean"}]}, {"key": "2019-12", "values": [{"count": 931941, "value": "Amazon.com"}, {"count": 383771, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 297446, "value": "GoDaddy.com, LLC"}, {"count": 279314, "value": "OVH SAS"}, {"count": 267574, "value": "Digital Ocean"}]}, {"key": "2020-01", "values": [{"count": 1262641, "value": "Amazon.com"}, {"count": 398543, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 301232, "value": "GoDaddy.com, LLC"}, {"count": 278669, "value": "Digital Ocean"}, {"count": 271222, "value": "OVH SAS"}]}, {"key": "2020-02", "values": [{"count": 1354436, "value": "Amazon.com"}, {"count": 393495, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 300019, "value": "GoDaddy.com, LLC"}, {"count": 286201, "value": "Digital Ocean"}, {"count": 270344, "value": "OVH SAS"}]}, {"key": "2020-03", "values": [{"count": 1380734, "value": "Amazon.com"}, {"count": 409176, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 306150, "value": "GoDaddy.com, LLC"}, {"count": 292495, "value": "Digital Ocean"}, {"count": 275636, "value": "OVH SAS"}]}, {"key": "2020-04", "values": [{"count": 1386450, "value": "Amazon.com"}, {"count": 415739, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 399128, "value": "OVH SAS"}, {"count": 299638, "value": "GoDaddy.com, LLC"}, {"count": 291841, "value": "Digital Ocean"}]}, {"key": "2020-05", "values": [{"count": 1349135, "value": "Amazon.com"}, {"count": 435788, "value": "OVH SAS"}, {"count": 406745, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 290414, "value": "GoDaddy.com, LLC"}, {"count": 290377, "value": "Digital Ocean"}]}, {"key": "2020-06", "values": [{"count": 1302706, "value": "Amazon.com"}, {"count": 426077, "value": "OVH SAS"}, {"count": 398630, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 286437, "value": "Digital Ocean"}, {"count": 285672, "value": "GoDaddy.com, LLC"}]}, {"key": "2020-07", "values": [{"count": 1364990, "value": "Amazon.com"}, {"count": 437106, "value": "OVH SAS"}, {"count": 409767, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 295121, "value": "Digital Ocean"}, {"count": 274220, "value": "GoDaddy.com, LLC"}]}, {"key": "2020-08", "values": [{"count": 1398296, "value": "Amazon.com"}, {"count": 450167, "value": "OVH SAS"}, {"count": 415203, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 302620, "value": "Digital Ocean"}, {"count": 275560, "value": "GoDaddy.com, LLC"}]}, {"key": "2020-09", "values": [{"count": 1412982, "value": "Amazon.com"}, {"count": 451870, "value": "OVH SAS"}, {"count": 414989, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 309730, "value": "Digital Ocean"}, {"count": 274839, "value": "GoDaddy.com, LLC"}]}, {"key": "2020-10", "values": [{"count": 1425059, "value": "Amazon.com"}, {"count": 455235, "value": "OVH SAS"}, {"count": 409558, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 310552, "value": "Digital Ocean"}, {"count": 274044, "value": "GoDaddy.com, LLC"}]}, {"key": "2020-11", "values": [{"count": 1425847, "value": "Amazon.com"}, {"count": 456518, "value": "OVH SAS"}, {"count": 409131, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 310929, "value": "Digital Ocean"}, {"count": 273213, "value": "GoDaddy.com, LLC"}]}, {"key": "2020-12", "values": [{"count": 1473044, "value": "Amazon.com"}, {"count": 465330, "value": "OVH SAS"}, {"count": 410416, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 316910, "value": "Digital Ocean"}, {"count": 265383, "value": "GoDaddy.com, LLC"}]}, {"key": "2021-01", "values": [{"count": 1499077, "value": "Amazon.com"}, {"count": 463033, "value": "OVH SAS"}, {"count": 404187, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 319920, "value": "Digital Ocean"}, {"count": 264268, "value": "GoDaddy.com, LLC"}]}, {"key": "2021-02", "values": [{"count": 1482977, "value": "Amazon.com"}, {"count": 455470, "value": "OVH SAS"}, {"count": 398207, "value": "Hangzhou Alibaba Advertising Co.,Ltd."}, {"count": 316051, "value": "Digital Ocean"}, {"count": 262989, "value": "GoDaddy.com, LLC"}]}, {"key": "2021-03", "values": [{"count": 554687, "value": "Amazon Technologies Inc."}, {"count": 381339, "value": "Aliyun Computing Co., LTD"}, {"count": 340173, "value": "DigitalOcean, LLC"}, {"count": 313883, "value": "GoDaddy.com, LLC"}, {"count": 222149, "value": "OVH SAS"}]}, {"key": "2021-04", "values": [{"count": 582193, "value": "Amazon Technologies Inc."}, {"count": 385003, "value": "Aliyun Computing Co., LTD"}, {"count": 350810, "value": "DigitalOcean, LLC"}, {"count": 316053, "value": "GoDaddy.com, LLC"}, {"count": 189510, "value": "Unified Layer"}]}, {"key": "2021-05", "values": [{"count": 591248, "value": "Amazon Technologies Inc."}, {"count": 376304, "value": "Aliyun Computing Co., LTD"}, {"count": 348660, "value": "DigitalOcean, LLC"}, {"count": 314115, "value": "GoDaddy.com, LLC"}, {"count": 194150, "value": "OVH SAS"}]}, {"key": "2021-06", "values": [{"count": 579879, "value": "Amazon Technologies Inc."}, {"count": 371634, "value": "Aliyun Computing Co., LTD"}, {"count": 350357, "value": "DigitalOcean, LLC"}, {"count": 313114, "value": "GoDaddy.com, LLC"}, {"count": 194191, "value": "OVH SAS"}]}, {"key": "2021-07", "values": [{"count": 571292, "value": "Amazon Technologies Inc."}, {"count": 359140, "value": "Aliyun Computing Co., LTD"}, {"count": 353578, "value": "DigitalOcean, LLC"}, {"count": 311947, "value": "GoDaddy.com, LLC"}, {"count": 193145, "value": "OVH SAS"}]}, {"key": "2021-08", "values": [{"count": 556965, "value": "Amazon Technologies Inc."}, {"count": 357475, "value": "Aliyun Computing Co., LTD"}, {"count": 351879, "value": "DigitalOcean, LLC"}, {"count": 276912, "value": "GoDaddy.com, LLC"}, {"count": 192031, "value": "OVH SAS"}]}, {"key": "2021-09", "values": [{"count": 552095, "value": "Amazon Technologies Inc."}, {"count": 359507, "value": "Aliyun Computing Co., LTD"}, {"count": 353471, "value": "DigitalOcean, LLC"}, {"count": 278726, "value": "GoDaddy.com, LLC"}, {"count": 192518, "value": "OVH SAS"}]}, {"key": "2021-10", "values": [{"count": 559827, "value": "Amazon Technologies Inc."}, {"count": 358821, "value": "DigitalOcean, LLC"}, {"count": 347176, "value": "Aliyun Computing Co., LTD"}, {"count": 280912, "value": "GoDaddy.com, LLC"}, {"count": 193157, "value": "OVH SAS"}]}, {"key": "2021-11", "values": [{"count": 554200, "value": "Amazon Technologies Inc."}, {"count": 355486, "value": "DigitalOcean, LLC"}, {"count": 349324, "value": "Aliyun Computing Co., LTD"}, {"count": 281005, "value": "GoDaddy.com, LLC"}, {"count": 194280, "value": "OVH SAS"}]}, {"key": "2021-12", "values": [{"count": 563733, "value": "Amazon Technologies Inc."}, {"count": 352412, "value": "DigitalOcean, LLC"}, {"count": 343369, "value": "Aliyun Computing Co., LTD"}, {"count": 281225, "value": "GoDaddy.com, LLC"}, {"count": 194984, "value": "OVH SAS"}]}, {"key": "2022-01", "values": [{"count": 569084, "value": "Amazon Technologies Inc."}, {"count": 353916, "value": "DigitalOcean, LLC"}, {"count": 343634, "value": "Aliyun Computing Co., LTD"}, {"count": 280717, "value": "GoDaddy.com, LLC"}, {"count": 194401, "value": "OVH SAS"}]}, {"key": "2022-02", "values": [{"count": 553324, "value": "Amazon Technologies Inc."}, {"count": 352789, "value": "DigitalOcean, LLC"}, {"count": 337964, "value": "Aliyun Computing Co., LTD"}, {"count": 192827, "value": "GoDaddy.com, LLC"}, {"count": 192565, "value": "OVH SAS"}]}, {"key": "2022-03", "values": [{"count": 559235, "value": "Amazon Technologies Inc."}, {"count": 353600, "value": "DigitalOcean, LLC"}, {"count": 344705, "value": "Aliyun Computing Co., LTD"}, {"count": 193109, "value": "OVH SAS"}, {"count": 193067, "value": "Amazon.com, Inc."}]}, {"key": "2022-04", "values": [{"count": 542062, "value": "Amazon Technologies Inc."}, {"count": 351197, "value": "DigitalOcean, LLC"}, {"count": 342262, "value": "Aliyun Computing Co., LTD"}, {"count": 193092, "value": "OVH SAS"}, {"count": 192537, "value": "GoDaddy.com, LLC"}]}, {"key": "2022-05", "values": [{"count": 547643, "value": "Amazon Technologies Inc."}, {"count": 351715, "value": "DigitalOcean, LLC"}, {"count": 332557, "value": "Aliyun Computing Co., LTD"}, {"count": 199571, "value": "Amazon.com, Inc."}, {"count": 194859, "value": "OVH SAS"}]}, {"key": "2022-06", "values": [{"count": 529742, "value": "Amazon Technologies Inc."}, {"count": 346499, "value": "DigitalOcean, LLC"}, {"count": 327084, "value": "Aliyun Computing Co., LTD"}, {"count": 192225, "value": "GoDaddy.com, LLC"}, {"count": 190763, "value": "OVH SAS"}]}, {"key": "2022-07", "values": [{"count": 531091, "value": "Amazon Technologies Inc."}, {"count": 345831, "value": "DigitalOcean, LLC"}, {"count": 322109, "value": "Aliyun Computing Co., LTD"}, {"count": 199522, "value": "GoDaddy.com, LLC"}, {"count": 196519, "value": "Amazon.com, Inc."}]}, {"key": "2022-08", "values": [{"count": 517178, "value": "Amazon Technologies Inc."}, {"count": 341329, "value": "DigitalOcean, LLC"}, {"count": 317160, "value": "Aliyun Computing Co., LTD"}, {"count": 193665, "value": "Amazon.com, Inc."}, {"count": 190697, "value": "GoDaddy.com, LLC"}]}, {"key": "2022-09", "values": [{"count": 505031, "value": "Amazon Technologies Inc."}, {"count": 338535, "value": "DigitalOcean, LLC"}, {"count": 313073, "value": "Aliyun Computing Co., LTD"}, {"count": 199258, "value": "Amazon.com, Inc."}, {"count": 195449, "value": "GoDaddy.com, LLC"}]}, {"key": "2022-10", "values": [{"count": 483309, "value": "Amazon Technologies Inc."}, {"count": 332493, "value": "DigitalOcean, LLC"}, {"count": 305202, "value": "Aliyun Computing Co., LTD"}, {"count": 192532, "value": "GoDaddy.com, LLC"}, {"count": 189646, "value": "Amazon.com, Inc."}]}, {"key": "2022-11", "values": [{"count": 486546, "value": "Amazon Technologies Inc."}, {"count": 332674, "value": "DigitalOcean, LLC"}, {"count": 300771, "value": "Aliyun Computing Co., LTD"}, {"count": 196558, "value": "Amazon.com, Inc."}, {"count": 192652, "value": "GoDaddy.com, LLC"}]}, {"key": "2022-12", "values": [{"count": 473865, "value": "Amazon Technologies Inc."}, {"count": 330478, "value": "DigitalOcean, LLC"}, {"count": 290707, "value": "Aliyun Computing Co., LTD"}, {"count": 199122, "value": "GoDaddy.com, LLC"}, {"count": 187630, "value": "Amazon.com, Inc."}]}, {"key": "2023-01", "values": [{"count": 462093, "value": "Amazon Technologies Inc."}, {"count": 328030, "value": "DigitalOcean, LLC"}, {"count": 282089, "value": "Aliyun Computing Co., LTD"}, {"count": 204884, "value": "Amazon.com, Inc."}, {"count": 192318, "value": "GoDaddy.com, LLC"}]}, {"key": "2023-02", "values": [{"count": 451901, "value": "Amazon Technologies Inc."}, {"count": 322244, "value": "DigitalOcean, LLC"}, {"count": 261852, "value": "Aliyun Computing Co., LTD"}, {"count": 190688, "value": "GoDaddy.com, LLC"}, {"count": 190536, "value": "Amazon.com, Inc."}]}, {"key": "2023-03", "values": [{"count": 456945, "value": "Amazon Technologies Inc."}, {"count": 320078, "value": "DigitalOcean, LLC"}, {"count": 198927, "value": "Aliyun Computing Co., LTD"}, {"count": 192876, "value": "GoDaddy.com, LLC"}, {"count": 190210, "value": "Amazon.com, Inc."}]}, {"key": "2023-04", "values": [{"count": 450213, "value": "Amazon Technologies Inc."}, {"count": 313221, "value": "DigitalOcean, LLC"}, {"count": 225063, "value": "Amazon.com, Inc."}, {"count": 192602, "value": "GoDaddy.com, LLC"}, {"count": 192190, "value": "Aliyun Computing Co., LTD"}]}, {"key": "2023-05", "values": [{"count": 455234, "value": "Amazon Technologies Inc."}, {"count": 314922, "value": "DigitalOcean, LLC"}, {"count": 219114, "value": "Amazon.com, Inc."}, {"count": 194330, "value": "GoDaddy.com, LLC"}, {"count": 190928, "value": "Aliyun Computing Co., LTD"}]}, {"key": "2023-06", "values": [{"count": 431454, "value": "Amazon Technologies Inc."}, {"count": 305862, "value": "DigitalOcean, LLC"}, {"count": 216374, "value": "Amazon.com, Inc."}, {"count": 192767, "value": "GoDaddy.com, LLC"}, {"count": 182156, "value": "Aliyun Computing Co., LTD"}]}, {"key": "2023-07", "values": [{"count": 428570, "value": "Amazon Technologies Inc."}, {"count": 303936, "value": "DigitalOcean, LLC"}, {"count": 225109, "value": "Amazon.com, Inc."}, {"count": 182745, "value": "GoDaddy.com, LLC"}, {"count": 179688, "value": "Aliyun Computing Co., LTD"}]}, {"key": "2023-08", "values": [{"count": 233980, "value": "Amazon Technologies Inc."}, {"count": 180242, "value": "DigitalOcean, LLC"}, {"count": 104302, "value": "Amazon.com, Inc."}, {"count": 100673, "value": "Aliyun Computing Co., LTD"}, {"count": 97769, "value": "Unified Layer"}]}]}}"#)
            .create();
    }

    let query = "".to_string();
    let facets = "".to_string();
    let (sender, receiver) = mpsc::channel();

    let mut app = App::new(query, facets, receiver);
    let mut state: AppState = AppState {
        unfocused: true,
        submitted: false,
        first_render: true,
        facet_indexes: HashMap::new(),
        app_log: String::new(),
        sender,
    };

    let backend: TestBackend = TestBackend::new(140, 40);
    let mut terminal = Terminal::new(backend)?;

    // First launch app
    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
        state.first_render = false;

        ui::render(&mut app, &mut state, frame);
    })?;

    let expected = Buffer::with_lines(vec![
        "┌──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐",
        "│ Query:                                                                                                                                   │",
        "│ Facets (optional):                                                                                                                       │",
        "└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘",
        "┌Info──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                               Make search by `Enter` a query in search box.                                              │",
        "│                                      Press `Ctrl-C` to stop running, switch between panels by `Tab`                                      │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘",
        "                                                                                                                                            ",
        "Switch panels [⇥]  Exit [^C]                                                                                                                ",
    ]);
    println!("{:?}", terminal.backend().buffer());
    terminal.backend().assert_buffer(&expected);

    // Focus searchbox
    app.switch_widgets(&mut state, false)?;
    assert!(app.search_input.focused());

    // Enter query
    for c in "nginx".chars() {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    // Focus facets input
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::empty())),
        &mut app,
        &mut state,
    )?;
    assert!(app.facets_input.focused());

    // Enter facets
    for c in "os:5".chars() {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    // Re-render UI
    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    // Test styled TUI
    // let mut expected = Buffer::with_lines(vec![
    //     "┌──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐",
    //     "│ Query: nginx                                                                                                                             │",
    //     "│ Facets (optional): os:5                                                                                                                  │",
    //     "└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘",
    //     "┌Info──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                               Make search by `Enter` a query in search box.                                              │",
    //     "│                                      Press `Ctrl-C` to stop running, switch between panels by `Tab`                                      │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘",
    //     "                                                                                                                                            ",
    //     "Search [⏎]  Move cursor [←→]  Delete Char [⌫]  Up/ Down [↑↓]  Unfocused [⎋]  Switch panels [⇥]  Exit [^C]                                   ",
    // ]);

    // // Style searchbox buffer
    // for i in 0..=139 {
    //     expected
    //         .get_mut(i, 0)
    //         .set_style(Style::default().fg(Color::Yellow));
    // }

    // expected
    //     .get_mut(0, 1)
    //     .set_style(Style::default().fg(Color::Yellow));

    // expected
    //     .get_mut(139, 1)
    //     .set_style(Style::default().fg(Color::Yellow));

    // expected
    //     .get_mut(0, 2)
    //     .set_style(Style::default().fg(Color::Yellow));

    // expected
    //     .get_mut(139, 2)
    //     .set_style(Style::default().fg(Color::Yellow));

    // for i in 2..=19 {
    //     expected
    //         .get_mut(i, 2)
    //         .set_style(Style::default().add_modifier(Modifier::BOLD));
    // }

    // for i in 0..=139 {
    //     expected
    //         .get_mut(i, 3)
    //         .set_style(Style::default().fg(Color::Yellow));
    // }

    // terminal.backend().assert_buffer(&expected);

    // Otherwise just check some rendered text in the buffer
    // Note that: println! only output on cargo test -- --nocapture
    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("Query: nginx"));
    assert!(buffer_str.contains("Facets (optional): os:5"));
    assert!(buffer_str.contains("Make search by `Enter` a query in search box."));
    assert!(buffer_str.contains("Switch panels [⇥]"));

    // Make search
    search_and_render(&mut app, &mut state, &mut terminal)?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("[x] query=nginx&facets=os"));
    assert!(buffer_str.contains("[x] Linux"));
    assert!(buffer_str.contains("[x] Ubuntu"));
    assert!(buffer_str.contains("[ ] Windows"));
    assert!(buffer_str.contains("Jun 2017"));
    assert!(buffer_str.contains("Up/ Down [↑↓]"));
    assert!(!buffer_str.contains("Export [^E]"));

    // Unfocus searchbox will show Export keybinding
    app.switch_widgets(&mut state, false)?;

    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("Export [^E]"));

    // Export chart data
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Char('E'), KeyModifiers::CONTROL)),
        &mut app,
        &mut state,
    )?;

    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("Exported chart to ./data.csv"));

    // Clear application log on next rendering
    app.ticks = 100;

    // We currently in Saved queries block, uncheck first MultiStatefulList line
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty())),
        &mut app,
        &mut state,
    )?;

    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(!buffer_str.contains("Exported chart to ./data.csv"));
    assert!(buffer_str.contains("[ ] query=nginx&facets=os"));

    // Re-export chart, here empty data
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Char('E'), KeyModifiers::CONTROL)),
        &mut app,
        &mut state,
    )?;

    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(!buffer_str.contains("No chart to export"));
    app.ticks = 100;

    app.select_widget(0);

    // Clear query input
    for _ in 0..20 {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    // Make search with empty query
    search_and_render(&mut app, &mut state, &mut terminal)?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(app.search_input.get_input().is_empty());
    assert!(buffer_str.contains("Invalid search query"));

    // Enter valid query with no resuts
    for c in "port:111222".chars() {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    app.select_widget(1);

    // Invalid search facet
    for _ in 0..20 {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    for c in "orggg".chars() {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    search_and_render(&mut app, &mut state, &mut terminal)?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("Invalid search facet"));

    for _ in 0..2 {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    search_and_render(&mut app, &mut state, &mut terminal)?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("No results found"));

    // Back Tab to switch to previous panel
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::BackTab, KeyModifiers::empty())),
        &mut app,
        &mut state,
    )?;

    for _ in 0..20 {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    for c in "apache port:80".chars() {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    search_and_render(&mut app, &mut state, &mut terminal)?;

    // Need one more render to see first saved queries checked
    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(!buffer_str.contains("No results found"));
    assert!(buffer_str.contains("[x] query=apache+port%3A8"));
    assert!(buffer_str.contains("[ ] query=nginx&facets=os"));
    assert!(buffer_str.contains("[x] Amazon.com"));
    assert!(app.line_chart.data[0].len() > 1);

    // Focus Facet values
    app.select_widget(3);
    assert!(app.facet_values.focused());

    // Unselect all lines
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::empty())),
        &mut app,
        &mut state,
    )?;

    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("[ ] Amazon.com"));
    assert!(app.line_chart.data[0].len() == 1);

    // Select all lines
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::empty())),
        &mut app,
        &mut state,
    )?;

    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("[x] Amazon.com"));
    assert!(buffer_str.contains("[x] Korea Telecom"));
    assert!(app.line_chart.data[0].len() > 1);

    // Export chart data again
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::CONTROL)),
        &mut app,
        &mut state,
    )?;

    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("Exported chart to ./data.csv"));
    assert!(app.facet_values.focused());
    assert!(!state.unfocused);
    assert!(app.running);

    // Unfocus all widgets
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty())),
        &mut app,
        &mut state,
    )?;

    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(!buffer_str.contains("Up/ Down [↑↓]"));
    assert!(!app.facet_values.focused());
    assert!(state.unfocused);
    assert!(app.running);

    // Close application
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Char('C'), KeyModifiers::CONTROL)),
        &mut app,
        &mut state,
    )?;
    assert!(!app.running);

    Ok(())
}

fn search_and_render(
    app: &mut App,
    state: &mut AppState,
    terminal: &mut Terminal<TestBackend>,
) -> AppResult<()> {
    // Input Enter key
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty())),
        app,
        state,
    )?;

    // Waiting for API response
    while app.blocking > 0 {
        sleep(Duration::from_millis(app.tick_rate));
        let _ = app.tick();
    }

    // Render TUI
    terminal.draw(|frame| {
        ui::render(app, state, frame);
    })?;

    Ok(())
}

// Clone ratatui-0.22.0/src/backend/test.rs::buffer_view as I can't import it
fn buffer_view(buffer: &Buffer) -> String {
    let mut view = String::with_capacity(buffer.content.len() + buffer.area.height as usize * 3);
    for cells in buffer.content.chunks(buffer.area.width as usize) {
        let mut overwritten = vec![];
        let mut skip: usize = 0;
        view.push('"');
        for (x, c) in cells.iter().enumerate() {
            if skip == 0 {
                view.push_str(&c.symbol);
            } else {
                overwritten.push((x, &c.symbol));
            }
            skip = std::cmp::max(skip, c.symbol.width()).saturating_sub(1);
        }
        view.push('"');
        if !overwritten.is_empty() {
            write!(&mut view, " Hidden by multi-width symbols: {overwritten:?}").unwrap();
        }
        view.push('\n');
    }
    view
}
