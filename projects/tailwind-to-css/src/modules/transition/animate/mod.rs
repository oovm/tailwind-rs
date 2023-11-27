use super::*;

use self::animation::Animation;

mod animation;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindAnimate {
    kind: Animation,
}

impl Display for TailwindAnimate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "animate-{}", self.kind)
    }
}

impl TailwindInstance for TailwindAnimate {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let animation = match &self.kind {
            Animation::None => "none".to_string(),
            Animation::Spin => "spin 1s linear infinite;".to_string(),
            Animation::Ping => "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;".to_string(),
            Animation::Pulse => "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;".to_string(),
            Animation::Bounce => "bounce 1s infinite;".to_string(),
            Animation::Arbitrary(s) => s.get_properties(),
        };
        css_attributes! {
            "animation" => animation
        }
    }
    fn additional(&self, _: &TailwindBuilder) -> String {
        match &self.kind {
            Animation::None => "",
            Animation::Spin => "@keyframes spin {from{transform: rotate(0deg);}to{transform: rotate(360deg);}}",
            Animation::Ping => "@keyframes ping {75%,100%{transform:scale(2);opacity:0;}}",
            Animation::Pulse => "@keyframes pulse {0%,100%{opacity:1;}50%{opacity:.5;}}",
            Animation::Bounce => {"@key frames bounce {0%,100%{transform:translateY(-25%);animation-timing-function:cubic-bezier(0.8,0,1,1);}50%{transform:translateY(0);animation-timing-function:cubic-bezier(0,0,0.2,1);}}"},
            Animation::Arbitrary(_) => "",
        }
        .to_string()
    }
}

impl TailwindAnimate {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Animation::parse(pattern, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Animation::parse_arbitrary(arbitrary)? })
    }
}
