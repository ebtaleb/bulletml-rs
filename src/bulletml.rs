pub enum BarrageType {
    HorizontalBarrage,
    VerticalBarrage
}

pub enum AttributeType {
    Aim,
    Absolute,
    Relative,
    Sequence
}

pub enum BulletAttributes {
    Repeat { times: uint, action : Action },
    ChangeSpeed { direction: Speed, term: uint },
    ChangeDirection { direction: Direction, term: uint },
    Accel { horizontal: Option<Horizontal>,
            vertical: Option<Vertical>,
            term: uint },
    Wait(uint),
    Vanish
}

pub enum Action {
    ActionDef { label: Option<str>,
                actions : Vec<Action>,
                tofire : Vec<Fire>,
                contents : Vec<BulletAttributes>},
    ActionRef(Box<Action>)
}

pub enum Bullet {
    BulletDef { label: str,
                direction: Option<Direction>,
                speed: Option<Speed>,
                actions: Vec<Action>},
    BulletRef(Box<Bullet>)
}

pub enum Fire {
    FireDef { label: str,
              direction: Option<Direction>,
              speed: Option<Speed>,
              bullet: Bullet },
    FireRef(Box<Fire>)
}

pub struct Direction {
    dirtype : AttributeType,
    number: int
}

pub struct Speed {
    spetype : AttributeType,
    number: int
}

pub struct Horizontal  {
    spetype : AttributeType,
    number: int
}

pub struct Vertical {
    spetype : AttributeType,
    number: int
}

pub struct BulletMLBody {
    attribute : BarrageType,
    tofire : Vec<Fire>,
    bullets : Vec<Bullet>,
    actions : Vec<Action>
}

