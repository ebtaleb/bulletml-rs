mod bulletml {

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

    pub enum Bullet {
        BulletDef { label: str,
                    direction: Option<Direction>,
                    speed: Option<Speed>,
                    actions: Vec<Action>},
        BulletRef(Box<Bullet>)
    }

    enum BulletAttributes {
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
                    contents : Vec<BulletMLBodyContent>},
        ActionRef(Box<Action>)
    }

    pub enum Fire {
        FireDef { label: str,
                  direction: Option<Int>,
                  speed: Option<Int>,
                  bullet: Bullet },
        FireRef(Box<Fire>)
    }

    struct Direction {
        dirtype : AttributeType,
        number: int
    }

    struct Speed {
        spetype : AttributeType,
        number: int
    }

    struct Horizontal  {
        spetype : AttributeType,
        number: int
    }

    struct Vertical {
        spetype : AttributeType,
        number: int
    }

    pub struct BulletMLBody {
        attribute : BarrageType,
        contents : Vec<BulletMLBodyContent>
    }

}

