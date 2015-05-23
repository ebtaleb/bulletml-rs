#[derive(Debug)]
pub enum BarrageType {
    HorizontalBarrage,
    VerticalBarrage,
    NoBarrage
}

#[derive(Debug)]
pub enum AttributeType {
    Aim,
    Absolute,
    Relative,
    Sequence
}

#[derive(Debug)]
pub enum BulletAttributes {
    Repeat { times: u32, action : Action },
    ChangeSpeed { direction: Speed, term: u32 },
    ChangeDirection { direction: Direction, term: u32 },
    Accel { horizontal: Option<Horizontal>,
            vertical: Option<Vertical>,
            term: u32 },
    Wait(u32),
    Vanish
}

#[derive(Debug)]
pub enum Action {
    ActionDef { label: Option<String>,
                actions : Vec<Action>,
                tofire : Vec<Fire>,
                contents : Vec<BulletAttributes>},
    ActionRef(Box<Action>)
}

#[derive(Debug)]
pub enum Bullet {
    BulletDef { label: Option<String>,
                direction: Option<Direction>,
                speed: Option<Speed>,
                actions: Vec<Action>},
    BulletRef(Box<Bullet>),
    NoBullet
}

#[derive(Debug)]
pub enum Fire {
    FireDef { label: Option<String>,
              direction: Option<Direction>,
              speed: Option<Speed>,
              bullet: Bullet },
    FireRef(Box<Fire>)
}

#[derive(Debug)]
pub struct Direction {
    dirtype : AttributeType,
    number: i32
}

#[derive(Debug)]
pub struct Speed {
    spetype : AttributeType,
    number: i32
}

#[derive(Debug)]
pub struct Horizontal  {
    spetype : AttributeType,
    number: i32
}

#[derive(Debug)]
pub struct Vertical {
    spetype : AttributeType,
    number: i32
}

#[derive(Debug)]
pub struct BulletMLBody {
    pub attribute : BarrageType,
    pub tofire : Vec<Fire>,
    pub bullets : Vec<Bullet>,
    pub actions : Vec<Action>
}

