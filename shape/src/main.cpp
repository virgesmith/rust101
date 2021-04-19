// compile and run with
// g++ -Wall -Werror -pedantic -std=c++17 src/main.cpp -o shape_cpp && ./shape_cpp

#include <memory>
#include <vector>
#include <cstdio> // closer to println!
#include <cmath>
#include <cstdint>

struct Shape
{
  virtual ~Shape() { }
  virtual double area() const = 0;
  virtual double aspect() const = 0;
};

template<typename T>
struct Circle : Shape
{
  Circle(T r) : m_r(r) { }

  double area() const
  {
    return static_cast<double>(m_r) * static_cast<double>(m_r) * M_PI;
  }

  double aspect() const
  {
    return 1.0;
  }

private:
  T m_r;
};


template<typename T>
struct Rectangle final : Shape
{
  Rectangle(T w, T h) : m_w(w), m_h(h) { }

  double area() const
  {
    return static_cast<double>(m_w) * static_cast<double>(m_h);
  }

  double aspect() const
  {
    return static_cast<double>(m_w) / static_cast<double>(m_h);
  }

private:
  T m_w;
  T m_h;
};


int main()
{
  std::vector<std::unique_ptr<Shape>> shapes; //{
  shapes.emplace_back(std::make_unique<Circle<int>>(1));
  shapes.emplace_back(std::make_unique<Rectangle<double>>(1.0, 3.14));

  for(const auto& s: shapes)
  {
    std::printf("area=%.3f aspect=%.3f\n", s->area(), s->aspect());
  }
}
