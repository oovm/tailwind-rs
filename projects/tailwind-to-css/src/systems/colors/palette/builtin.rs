use super::*;
/// Builtin colors
/// https://tailwindcss.com/docs/customizing-colors
impl Palette {
    /// ## SLATE
    /// <span style="color:#F8FAFC">50</span>,
    /// <span style="color:#F1F5F9">100</span>,
    /// <span style="color:#E2E8F0">200</span>,
    /// <span style="color:#CBD5E1">300</span>,
    /// <span style="color:#94A3B8">400</span>,
    /// <span style="color:#64748B">500</span>,
    /// <span style="color:#475569">600</span>,
    /// <span style="color:#334155">700</span>,
    /// <span style="color:#1E293B">800</span>,
    /// <span style="color:#0F172A">900</span>
    pub fn slate() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#F8FAFC").unwrap());
        colors.insert(100, Srgb::from_str("#F1F5F9").unwrap());
        colors.insert(200, Srgb::from_str("#E2E8F0").unwrap());
        colors.insert(300, Srgb::from_str("#CBD5E1").unwrap());
        colors.insert(400, Srgb::from_str("#94A3B8").unwrap());
        colors.insert(500, Srgb::from_str("#64748B").unwrap());
        colors.insert(600, Srgb::from_str("#475569").unwrap());
        colors.insert(700, Srgb::from_str("#334155").unwrap());
        colors.insert(800, Srgb::from_str("#1E293B").unwrap());
        colors.insert(900, Srgb::from_str("#0F172A").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## GRAY
    /// <span style="color:#F9FAFB">50</span>,
    /// <span style="color:#F3F4F6">100</span>,
    /// <span style="color:#E5E7EB">200</span>,
    /// <span style="color:#D1D5DB">300</span>,
    /// <span style="color:#9CA3AF">400</span>,
    /// <span style="color:#6B7280">500</span>,
    /// <span style="color:#4B5563">600</span>,
    /// <span style="color:#374151">700</span>,
    /// <span style="color:#1F2937">800</span>,
    /// <span style="color:#111827">900</span>
    pub fn gray() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#F9FAFB").unwrap());
        colors.insert(100, Srgb::from_str("#F3F4F6").unwrap());
        colors.insert(200, Srgb::from_str("#E5E7EB").unwrap());
        colors.insert(300, Srgb::from_str("#D1D5DB").unwrap());
        colors.insert(400, Srgb::from_str("#9CA3AF").unwrap());
        colors.insert(500, Srgb::from_str("#6B7280").unwrap());
        colors.insert(600, Srgb::from_str("#4B5563").unwrap());
        colors.insert(700, Srgb::from_str("#374151").unwrap());
        colors.insert(800, Srgb::from_str("#1F2937").unwrap());
        colors.insert(900, Srgb::from_str("#111827").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## ZINC
    /// <span style="color:#FAFAFA">50</span>,
    /// <span style="color:#F4F4F5">100</span>,
    /// <span style="color:#E4E4E7">200</span>,
    /// <span style="color:#D4D4D8">300</span>,
    /// <span style="color:#A1A1AA">400</span>,
    /// <span style="color:#71717A">500</span>,
    /// <span style="color:#52525B">600</span>,
    /// <span style="color:#3F3F46">700</span>,
    /// <span style="color:#27272A">800</span>,
    /// <span style="color:#18181B">900</span>
    pub fn zinc() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#FAFAFA").unwrap());
        colors.insert(100, Srgb::from_str("#F4F4F5").unwrap());
        colors.insert(200, Srgb::from_str("#E4E4E7").unwrap());
        colors.insert(300, Srgb::from_str("#D4D4D8").unwrap());
        colors.insert(400, Srgb::from_str("#A1A1AA").unwrap());
        colors.insert(500, Srgb::from_str("#71717A").unwrap());
        colors.insert(600, Srgb::from_str("#52525B").unwrap());
        colors.insert(700, Srgb::from_str("#3F3F46").unwrap());
        colors.insert(800, Srgb::from_str("#27272A").unwrap());
        colors.insert(900, Srgb::from_str("#18181B").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## NEUTRAL
    /// <span style="color:#FAFAFA">50</span>,
    /// <span style="color:#F5F5F5">100</span>,
    /// <span style="color:#E5E5E5">200</span>,
    /// <span style="color:#D4D4D4">300</span>,
    /// <span style="color:#A3A3A3">400</span>,
    /// <span style="color:#737373">500</span>,
    /// <span style="color:#525252">600</span>,
    /// <span style="color:#404040">700</span>,
    /// <span style="color:#262626">800</span>,
    /// <span style="color:#171717">900</span>
    pub fn neutral() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#FAFAFA").unwrap());
        colors.insert(100, Srgb::from_str("#F5F5F5").unwrap());
        colors.insert(200, Srgb::from_str("#E5E5E5").unwrap());
        colors.insert(300, Srgb::from_str("#D4D4D4").unwrap());
        colors.insert(400, Srgb::from_str("#A3A3A3").unwrap());
        colors.insert(500, Srgb::from_str("#737373").unwrap());
        colors.insert(600, Srgb::from_str("#525252").unwrap());
        colors.insert(700, Srgb::from_str("#404040").unwrap());
        colors.insert(800, Srgb::from_str("#262626").unwrap());
        colors.insert(900, Srgb::from_str("#171717").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## STONE
    /// <span style="color:#FAFAF9">50</span>,
    /// <span style="color:#F5F5F4">100</span>,
    /// <span style="color:#E7E5E4">200</span>,
    /// <span style="color:#D6D3D1">300</span>,
    /// <span style="color:#A8A29E">400</span>,
    /// <span style="color:#78716C">500</span>,
    /// <span style="color:#57534E">600</span>,
    /// <span style="color:#44403C">700</span>,
    /// <span style="color:#292524">800</span>,
    /// <span style="color:#1C1917">900</span>
    pub fn stone() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#FAFAF9").unwrap());
        colors.insert(100, Srgb::from_str("#F5F5F4").unwrap());
        colors.insert(200, Srgb::from_str("#E7E5E4").unwrap());
        colors.insert(300, Srgb::from_str("#D6D3D1").unwrap());
        colors.insert(400, Srgb::from_str("#A8A29E").unwrap());
        colors.insert(500, Srgb::from_str("#78716C").unwrap());
        colors.insert(600, Srgb::from_str("#57534E").unwrap());
        colors.insert(700, Srgb::from_str("#44403C").unwrap());
        colors.insert(800, Srgb::from_str("#292524").unwrap());
        colors.insert(900, Srgb::from_str("#1C1917").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## RED
    /// <span style="color:#FEF2F2">50</span>,
    /// <span style="color:#FEE2E2">100</span>,
    /// <span style="color:#FECACA">200</span>,
    /// <span style="color:#FCA5A5">300</span>,
    /// <span style="color:#F87171">400</span>,
    /// <span style="color:#EF4444">500</span>,
    /// <span style="color:#DC2626">600</span>,
    /// <span style="color:#B91C1C">700</span>,
    /// <span style="color:#991B1B">800</span>,
    /// <span style="color:#7F1D1D">900</span>
    pub fn red() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#FEF2F2").unwrap());
        colors.insert(100, Srgb::from_str("#FEE2E2").unwrap());
        colors.insert(200, Srgb::from_str("#FECACA").unwrap());
        colors.insert(300, Srgb::from_str("#FCA5A5").unwrap());
        colors.insert(400, Srgb::from_str("#F87171").unwrap());
        colors.insert(500, Srgb::from_str("#EF4444").unwrap());
        colors.insert(600, Srgb::from_str("#DC2626").unwrap());
        colors.insert(700, Srgb::from_str("#B91C1C").unwrap());
        colors.insert(800, Srgb::from_str("#991B1B").unwrap());
        colors.insert(900, Srgb::from_str("#7F1D1D").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## ORANGE
    /// <span style="color:#FFF7ED">50</span>,
    /// <span style="color:#FFEDD5">100</span>,
    /// <span style="color:#FED7AA">200</span>,
    /// <span style="color:#FDBA74">300</span>,
    /// <span style="color:#FB923C">400</span>,
    /// <span style="color:#F97316">500</span>,
    /// <span style="color:#EA580C">600</span>,
    /// <span style="color:#C2410C">700</span>,
    /// <span style="color:#9A3412">800</span>,
    /// <span style="color:#7C2D12">900</span>
    pub fn orange() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#FFF7ED").unwrap());
        colors.insert(100, Srgb::from_str("#FFEDD5").unwrap());
        colors.insert(200, Srgb::from_str("#FED7AA").unwrap());
        colors.insert(300, Srgb::from_str("#FDBA74").unwrap());
        colors.insert(400, Srgb::from_str("#FB923C").unwrap());
        colors.insert(500, Srgb::from_str("#F97316").unwrap());
        colors.insert(600, Srgb::from_str("#EA580C").unwrap());
        colors.insert(700, Srgb::from_str("#C2410C").unwrap());
        colors.insert(800, Srgb::from_str("#9A3412").unwrap());
        colors.insert(900, Srgb::from_str("#7C2D12").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## AMBER
    /// <span style="color:#FFFBEB">50</span>,
    /// <span style="color:#FEF3C7">100</span>,
    /// <span style="color:#FDE68A">200</span>,
    /// <span style="color:#FCD34D">300</span>,
    /// <span style="color:#FBBF24">400</span>,
    /// <span style="color:#F59E0B">500</span>,
    /// <span style="color:#D97706">600</span>,
    /// <span style="color:#B45309">700</span>,
    /// <span style="color:#92400E">800</span>,
    /// <span style="color:#78350F">900</span>
    pub fn amber() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#FFFBEB").unwrap());
        colors.insert(100, Srgb::from_str("#FEF3C7").unwrap());
        colors.insert(200, Srgb::from_str("#FDE68A").unwrap());
        colors.insert(300, Srgb::from_str("#FCD34D").unwrap());
        colors.insert(400, Srgb::from_str("#FBBF24").unwrap());
        colors.insert(500, Srgb::from_str("#F59E0B").unwrap());
        colors.insert(600, Srgb::from_str("#D97706").unwrap());
        colors.insert(700, Srgb::from_str("#B45309").unwrap());
        colors.insert(800, Srgb::from_str("#92400E").unwrap());
        colors.insert(900, Srgb::from_str("#78350F").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## YELLOW
    /// <span style="color:#FEFCE8">50</span>,
    /// <span style="color:#FEF9C3">100</span>,
    /// <span style="color:#FEF08A">200</span>,
    /// <span style="color:#FDE047">300</span>,
    /// <span style="color:#FACC15">400</span>,
    /// <span style="color:#EAB308">500</span>,
    /// <span style="color:#CA8A04">600</span>,
    /// <span style="color:#A16207">700</span>,
    /// <span style="color:#854D0E">800</span>,
    /// <span style="color:#713F12">900</span>
    pub fn yellow() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#FEFCE8").unwrap());
        colors.insert(100, Srgb::from_str("#FEF9C3").unwrap());
        colors.insert(200, Srgb::from_str("#FEF08A").unwrap());
        colors.insert(300, Srgb::from_str("#FDE047").unwrap());
        colors.insert(400, Srgb::from_str("#FACC15").unwrap());
        colors.insert(500, Srgb::from_str("#EAB308").unwrap());
        colors.insert(600, Srgb::from_str("#CA8A04").unwrap());
        colors.insert(700, Srgb::from_str("#A16207").unwrap());
        colors.insert(800, Srgb::from_str("#854D0E").unwrap());
        colors.insert(900, Srgb::from_str("#713F12").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## LIME
    /// <span style="color:#F7FEE7">50</span>,
    /// <span style="color:#ECFCCB">100</span>,
    /// <span style="color:#D9F99D">200</span>,
    /// <span style="color:#BEF264">300</span>,
    /// <span style="color:#A3E635">400</span>,
    /// <span style="color:#84CC16">500</span>,
    /// <span style="color:#65A30D">600</span>,
    /// <span style="color:#4D7C0F">700</span>,
    /// <span style="color:#3F6212">800</span>,
    /// <span style="color:#365314">900</span>
    pub fn lime() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#F7FEE7").unwrap());
        colors.insert(100, Srgb::from_str("#ECFCCB").unwrap());
        colors.insert(200, Srgb::from_str("#D9F99D").unwrap());
        colors.insert(300, Srgb::from_str("#BEF264").unwrap());
        colors.insert(400, Srgb::from_str("#A3E635").unwrap());
        colors.insert(500, Srgb::from_str("#84CC16").unwrap());
        colors.insert(600, Srgb::from_str("#65A30D").unwrap());
        colors.insert(700, Srgb::from_str("#4D7C0F").unwrap());
        colors.insert(800, Srgb::from_str("#3F6212").unwrap());
        colors.insert(900, Srgb::from_str("#365314").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## GREEN
    /// <span style="color:#F0FDF4">50</span>,
    /// <span style="color:#DCFCE7">100</span>,
    /// <span style="color:#BBF7D0">200</span>,
    /// <span style="color:#86EFAC">300</span>,
    /// <span style="color:#4ADE80">400</span>,
    /// <span style="color:#22C55E">500</span>,
    /// <span style="color:#16A34A">600</span>,
    /// <span style="color:#15803D">700</span>,
    /// <span style="color:#166534">800</span>,
    /// <span style="color:#14532D">900</span>
    pub fn green() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#F0FDF4").unwrap());
        colors.insert(100, Srgb::from_str("#DCFCE7").unwrap());
        colors.insert(200, Srgb::from_str("#BBF7D0").unwrap());
        colors.insert(300, Srgb::from_str("#86EFAC").unwrap());
        colors.insert(400, Srgb::from_str("#4ADE80").unwrap());
        colors.insert(500, Srgb::from_str("#22C55E").unwrap());
        colors.insert(600, Srgb::from_str("#16A34A").unwrap());
        colors.insert(700, Srgb::from_str("#15803D").unwrap());
        colors.insert(800, Srgb::from_str("#166534").unwrap());
        colors.insert(900, Srgb::from_str("#14532D").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## EMERALD
    /// <span style="color:#ECFDF5">50</span>,
    /// <span style="color:#D1FAE5">100</span>,
    /// <span style="color:#A7F3D0">200</span>,
    /// <span style="color:#6EE7B7">300</span>,
    /// <span style="color:#34D399">400</span>,
    /// <span style="color:#10B981">500</span>,
    /// <span style="color:#059669">600</span>,
    /// <span style="color:#047857">700</span>,
    /// <span style="color:#065F46">800</span>,
    /// <span style="color:#064E3B">900</span>
    pub fn emerald() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#ECFDF5").unwrap());
        colors.insert(100, Srgb::from_str("#D1FAE5").unwrap());
        colors.insert(200, Srgb::from_str("#A7F3D0").unwrap());
        colors.insert(300, Srgb::from_str("#6EE7B7").unwrap());
        colors.insert(400, Srgb::from_str("#34D399").unwrap());
        colors.insert(500, Srgb::from_str("#10B981").unwrap());
        colors.insert(600, Srgb::from_str("#059669").unwrap());
        colors.insert(700, Srgb::from_str("#047857").unwrap());
        colors.insert(800, Srgb::from_str("#065F46").unwrap());
        colors.insert(900, Srgb::from_str("#064E3B").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## TEAL
    /// <span style="color:#F0FDFA">50</span>,
    /// <span style="color:#CCFBF1">100</span>,
    /// <span style="color:#99F6E4">200</span>,
    /// <span style="color:#5EEAD4">300</span>,
    /// <span style="color:#2DD4BF">400</span>,
    /// <span style="color:#14B8A6">500</span>,
    /// <span style="color:#0D9488">600</span>,
    /// <span style="color:#0F766E">700</span>,
    /// <span style="color:#115E59">800</span>,
    /// <span style="color:#134E4A">900</span>
    pub fn teal() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#F0FDFA").unwrap());
        colors.insert(100, Srgb::from_str("#CCFBF1").unwrap());
        colors.insert(200, Srgb::from_str("#99F6E4").unwrap());
        colors.insert(300, Srgb::from_str("#5EEAD4").unwrap());
        colors.insert(400, Srgb::from_str("#2DD4BF").unwrap());
        colors.insert(500, Srgb::from_str("#14B8A6").unwrap());
        colors.insert(600, Srgb::from_str("#0D9488").unwrap());
        colors.insert(700, Srgb::from_str("#0F766E").unwrap());
        colors.insert(800, Srgb::from_str("#115E59").unwrap());
        colors.insert(900, Srgb::from_str("#134E4A").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## CYAN
    /// <span style="color:#ECFEFF">50</span>,
    /// <span style="color:#CFFAFE">100</span>,
    /// <span style="color:#A5F3FC">200</span>,
    /// <span style="color:#67E8F9">300</span>,
    /// <span style="color:#22D3EE">400</span>,
    /// <span style="color:#06B6D4">500</span>,
    /// <span style="color:#0891B2">600</span>,
    /// <span style="color:#0E7490">700</span>,
    /// <span style="color:#155E75">800</span>,
    /// <span style="color:#164E63">900</span>
    pub fn cyan() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#ECFEFF").unwrap());
        colors.insert(100, Srgb::from_str("#CFFAFE").unwrap());
        colors.insert(200, Srgb::from_str("#A5F3FC").unwrap());
        colors.insert(300, Srgb::from_str("#67E8F9").unwrap());
        colors.insert(400, Srgb::from_str("#22D3EE").unwrap());
        colors.insert(500, Srgb::from_str("#06B6D4").unwrap());
        colors.insert(600, Srgb::from_str("#0891B2").unwrap());
        colors.insert(700, Srgb::from_str("#0E7490").unwrap());
        colors.insert(800, Srgb::from_str("#155E75").unwrap());
        colors.insert(900, Srgb::from_str("#164E63").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## SKY
    /// <span style="color:#F0F9FF">50</span>,
    /// <span style="color:#E0F2FE">100</span>,
    /// <span style="color:#BAE6FD">200</span>,
    /// <span style="color:#7DD3FC">300</span>,
    /// <span style="color:#38BDF8">400</span>,
    /// <span style="color:#0EA5E9">500</span>,
    /// <span style="color:#0284C7">600</span>,
    /// <span style="color:#0369A1">700</span>,
    /// <span style="color:#075985">800</span>,
    /// <span style="color:#0C4A6E">900</span>
    pub fn sky() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#F0F9FF").unwrap());
        colors.insert(100, Srgb::from_str("#E0F2FE").unwrap());
        colors.insert(200, Srgb::from_str("#BAE6FD").unwrap());
        colors.insert(300, Srgb::from_str("#7DD3FC").unwrap());
        colors.insert(400, Srgb::from_str("#38BDF8").unwrap());
        colors.insert(500, Srgb::from_str("#0EA5E9").unwrap());
        colors.insert(600, Srgb::from_str("#0284C7").unwrap());
        colors.insert(700, Srgb::from_str("#0369A1").unwrap());
        colors.insert(800, Srgb::from_str("#075985").unwrap());
        colors.insert(900, Srgb::from_str("#0C4A6E").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## BLUE
    /// <span style="color:#EFF6FF">50</span>,
    /// <span style="color:#DBEAFE">100</span>,
    /// <span style="color:#BFDBFE">200</span>,
    /// <span style="color:#93C5FD">300</span>,
    /// <span style="color:#60A5FA">400</span>,
    /// <span style="color:#3B82F6">500</span>,
    /// <span style="color:#2563EB">600</span>,
    /// <span style="color:#1D4ED8">700</span>,
    /// <span style="color:#1E40AF">800</span>,
    /// <span style="color:#1E3A8A">900</span>
    pub fn blue() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#EFF6FF").unwrap());
        colors.insert(100, Srgb::from_str("#DBEAFE").unwrap());
        colors.insert(200, Srgb::from_str("#BFDBFE").unwrap());
        colors.insert(300, Srgb::from_str("#93C5FD").unwrap());
        colors.insert(400, Srgb::from_str("#60A5FA").unwrap());
        colors.insert(500, Srgb::from_str("#3B82F6").unwrap());
        colors.insert(600, Srgb::from_str("#2563EB").unwrap());
        colors.insert(700, Srgb::from_str("#1D4ED8").unwrap());
        colors.insert(800, Srgb::from_str("#1E40AF").unwrap());
        colors.insert(900, Srgb::from_str("#1E3A8A").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## INDIGO
    /// <span style="color:#EEF2FF">50</span>,
    /// <span style="color:#E0E7FF">100</span>,
    /// <span style="color:#C7D2FE">200</span>,
    /// <span style="color:#A5B4FC">300</span>,
    /// <span style="color:#818CF8">400</span>,
    /// <span style="color:#6366F1">500</span>,
    /// <span style="color:#4F46E5">600</span>,
    /// <span style="color:#4338CA">700</span>,
    /// <span style="color:#3730A3">800</span>,
    /// <span style="color:#312E81">900</span>
    pub fn indigo() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#EEF2FF").unwrap());
        colors.insert(100, Srgb::from_str("#E0E7FF").unwrap());
        colors.insert(200, Srgb::from_str("#C7D2FE").unwrap());
        colors.insert(300, Srgb::from_str("#A5B4FC").unwrap());
        colors.insert(400, Srgb::from_str("#818CF8").unwrap());
        colors.insert(500, Srgb::from_str("#6366F1").unwrap());
        colors.insert(600, Srgb::from_str("#4F46E5").unwrap());
        colors.insert(700, Srgb::from_str("#4338CA").unwrap());
        colors.insert(800, Srgb::from_str("#3730A3").unwrap());
        colors.insert(900, Srgb::from_str("#312E81").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## VIOLET
    /// <span style="color:#F5F3FF">50</span>,
    /// <span style="color:#EDE9FE">100</span>,
    /// <span style="color:#DDD6FE">200</span>,
    /// <span style="color:#C4B5FD">300</span>,
    /// <span style="color:#A78BFA">400</span>,
    /// <span style="color:#8B5CF6">500</span>,
    /// <span style="color:#7C3AED">600</span>,
    /// <span style="color:#6D28D9">700</span>,
    /// <span style="color:#5B21B6">800</span>,
    /// <span style="color:#4C1D95">900</span>
    pub fn violet() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#F5F3FF").unwrap());
        colors.insert(100, Srgb::from_str("#EDE9FE").unwrap());
        colors.insert(200, Srgb::from_str("#DDD6FE").unwrap());
        colors.insert(300, Srgb::from_str("#C4B5FD").unwrap());
        colors.insert(400, Srgb::from_str("#A78BFA").unwrap());
        colors.insert(500, Srgb::from_str("#8B5CF6").unwrap());
        colors.insert(600, Srgb::from_str("#7C3AED").unwrap());
        colors.insert(700, Srgb::from_str("#6D28D9").unwrap());
        colors.insert(800, Srgb::from_str("#5B21B6").unwrap());
        colors.insert(900, Srgb::from_str("#4C1D95").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## PURPLE
    /// <span style="color:#FAF5FF">50</span>,
    /// <span style="color:#F3E8FF">100</span>,
    /// <span style="color:#E9D5FF">200</span>,
    /// <span style="color:#D8B4FE">300</span>,
    /// <span style="color:#C084FC">400</span>,
    /// <span style="color:#A855F7">500</span>,
    /// <span style="color:#9333EA">600</span>,
    /// <span style="color:#7E22CE">700</span>,
    /// <span style="color:#6B21A8">800</span>,
    /// <span style="color:#581C87">900</span>
    pub fn purple() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#FAF5FF").unwrap());
        colors.insert(100, Srgb::from_str("#F3E8FF").unwrap());
        colors.insert(200, Srgb::from_str("#E9D5FF").unwrap());
        colors.insert(300, Srgb::from_str("#D8B4FE").unwrap());
        colors.insert(400, Srgb::from_str("#C084FC").unwrap());
        colors.insert(500, Srgb::from_str("#A855F7").unwrap());
        colors.insert(600, Srgb::from_str("#9333EA").unwrap());
        colors.insert(700, Srgb::from_str("#7E22CE").unwrap());
        colors.insert(800, Srgb::from_str("#6B21A8").unwrap());
        colors.insert(900, Srgb::from_str("#581C87").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## FUCHSIA
    /// <span style="color:#FDF4FF">50</span>,
    /// <span style="color:#FAE8FF">100</span>,
    /// <span style="color:#F5D0FE">200</span>,
    /// <span style="color:#F0ABFC">300</span>,
    /// <span style="color:#E879F9">400</span>,
    /// <span style="color:#D946EF">500</span>,
    /// <span style="color:#C026D3">600</span>,
    /// <span style="color:#A21CAF">700</span>,
    /// <span style="color:#86198F">800</span>,
    /// <span style="color:#701A75">900</span>
    pub fn fuchsia() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#FDF4FF").unwrap());
        colors.insert(100, Srgb::from_str("#FAE8FF").unwrap());
        colors.insert(200, Srgb::from_str("#F5D0FE").unwrap());
        colors.insert(300, Srgb::from_str("#F0ABFC").unwrap());
        colors.insert(400, Srgb::from_str("#E879F9").unwrap());
        colors.insert(500, Srgb::from_str("#D946EF").unwrap());
        colors.insert(600, Srgb::from_str("#C026D3").unwrap());
        colors.insert(700, Srgb::from_str("#A21CAF").unwrap());
        colors.insert(800, Srgb::from_str("#86198F").unwrap());
        colors.insert(900, Srgb::from_str("#701A75").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## PINK
    /// <span style="color:#FDF2F8">50</span>,
    /// <span style="color:#FCE7F3">100</span>,
    /// <span style="color:#FBCFE8">200</span>,
    /// <span style="color:#F9A8D4">300</span>,
    /// <span style="color:#F472B6">400</span>,
    /// <span style="color:#EC4899">500</span>,
    /// <span style="color:#DB2777">600</span>,
    /// <span style="color:#BE185D">700</span>,
    /// <span style="color:#9D174D">800</span>,
    /// <span style="color:#831843">900</span>
    pub fn pink() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#FDF2F8").unwrap());
        colors.insert(100, Srgb::from_str("#FCE7F3").unwrap());
        colors.insert(200, Srgb::from_str("#FBCFE8").unwrap());
        colors.insert(300, Srgb::from_str("#F9A8D4").unwrap());
        colors.insert(400, Srgb::from_str("#F472B6").unwrap());
        colors.insert(500, Srgb::from_str("#EC4899").unwrap());
        colors.insert(600, Srgb::from_str("#DB2777").unwrap());
        colors.insert(700, Srgb::from_str("#BE185D").unwrap());
        colors.insert(800, Srgb::from_str("#9D174D").unwrap());
        colors.insert(900, Srgb::from_str("#831843").unwrap());
        Self { gradient: true, key_points: colors }
    }
    /// ## ROSE
    /// <span style="color:#FFF1F2">50</span>,
    /// <span style="color:#FFE4E6">100</span>,
    /// <span style="color:#FECDD3">200</span>,
    /// <span style="color:#FDA4AF">300</span>,
    /// <span style="color:#FB7185">400</span>,
    /// <span style="color:#F43F5E">500</span>,
    /// <span style="color:#E11D48">600</span>,
    /// <span style="color:#BE123C">700</span>,
    /// <span style="color:#9F1239">800</span>,
    /// <span style="color:#881337">900</span>
    pub fn rose() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Srgb::from_str("#FFF1F2").unwrap());
        colors.insert(100, Srgb::from_str("#FFE4E6").unwrap());
        colors.insert(200, Srgb::from_str("#FECDD3").unwrap());
        colors.insert(300, Srgb::from_str("#FDA4AF").unwrap());
        colors.insert(400, Srgb::from_str("#FB7185").unwrap());
        colors.insert(500, Srgb::from_str("#F43F5E").unwrap());
        colors.insert(600, Srgb::from_str("#E11D48").unwrap());
        colors.insert(700, Srgb::from_str("#BE123C").unwrap());
        colors.insert(800, Srgb::from_str("#9F1239").unwrap());
        colors.insert(900, Srgb::from_str("#881337").unwrap());
        Self { gradient: true, key_points: colors }
    }
}
