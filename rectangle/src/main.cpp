// compile with
// g++ -Wall -g -std=c++14 src/main.cpp -o target/debug/rectangle_cpp

#include <memory>
#include <cstdio> // closer to println!
#include <cmath>
#include <cstdint>

struct Shape
{
  virtual double area() const = 0;
  virtual double aspect() const = 0;
};

template<typename T>
struct Rect : Shape
{
  Rect(T w_, T h_) : w(w_), h(h_) { }

  double area() const
  {
    return static_cast<double>(w) * static_cast<double>(h);
  }

  double aspect() const
  {
    return static_cast<double>(w) / static_cast<double>(h);
  }

private:
  T w;
  T h;
};

template<typename T>
struct Ellipse : Shape
{
  Ellipse(T rx_, T ry_) : rx(rx_), ry(ry_) { }

  double area() const
  {
    return static_cast<double>(rx) * static_cast<double>(ry) * M_PI;
  }

  double aspect() const
  {
    return static_cast<double>(rx) / static_cast<double>(ry);
  }

private:
  T rx;
  T ry;
};


int main() 
{
  std::unique_ptr<Shape> rect = std::make_unique<Rect<int32_t>>( 30, 50 );
  printf("The area of the rectangle is %f square pixels aspect %f.\n", 
     rect->area(), rect->aspect());

  std::unique_ptr<Shape> ell = std::make_unique<Ellipse<int32_t>>( 30, 50 );
  printf("The area of the ellipse is %f square - pixels aspect %f.\n", 
     ell->area(), ell->aspect());

  // Compile error from static cast note enforcement on construction unlike rust example
  //Ellipse<const char*>("hello", "world");
}
