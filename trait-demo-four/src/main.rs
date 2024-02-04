
trait Add<T> {
    type Output;
    fn add(self,rhs:T) -> Self::Output;
}

struct Point{
    x: i32,
    y: i32,
}


// 为 Point<T> 实现 Add<T> trait
impl Add<Point> for Point {
    type Output = Self;
    fn add(self, rhs:Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// 为 Point<T> 实现 Add<i32> trait
impl Add<i32> for Point {
    type Output = Self;
    fn add(self, rhs:i32) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}




trait Default {
    fn default() -> Self;
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 0,
            g: 0,
            b: 0,
        }
    }
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Color {
            r,
            g,
            b,
        }
    }
}

impl Color {
  fn red(r: u8) -> Self {
      Color {
          r: r,
          ..Color::default()
      }
  }

  fn green(g: u8) -> Self {
      Color {
          g: g,
          ..Color::default()
      }
  }
  fn blue(b:u8) -> Self {
      Color {
          b: b,
          ..Color::default()
      }
  }
}


fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1.add(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let p4 = Point { x: 1, y: 1 };
    let delta = 2;

    let p5 = p4.add(delta); // p4.add(delta) 等价于 Add::add(p4, delta)

    println!("p5.x = {}, p5.y = {}", p5.x, p5.y);


    let color = <Color as Default>::default();
    println!("color.r = {}, color.g = {}, color.b = {}", color.r, color.g, color.b);

    let color: Color = Default::default();
    println!("color.r = {}, color.g = {}, color.b = {}", color.r, color.g, color.b);

    // let mut color = Color::new(1,2,3);
    // Color::red(1);
    // Color.green(2);
    // Color.blue(3);


    let color = Color::red(1);
    // color = color.red(3);
    println!("color.r = {}, color.g = {}, color.b = {}", color.r, color.g, color.b);
}
