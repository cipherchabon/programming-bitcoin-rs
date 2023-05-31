# Elliptic Curves

Equation: `y^2 =x^3 + ax + b`

The elliptic curve used in Bitcoin is called `secp256k1` and it uses this particular equation: `y2 = x3 + 7`. The curve is defined by the constants `a = 0`, `b = 7`.

We are not interested in the curve itself, but specific points on the curve.

We can define the curve with just the two numbers `a` and `b`.

## Code Point Addition
To code point addition, we’re going to split it up into three steps:
1. Where the points are in a vertical line or using the identity point
    - `x1 == x2 && y1 != y2`
2. Where the points are not in a vertical line, but are different
    - `x1 != x2`
3. Where the two points are the same
    - `x1 == x2 && y1 == y2`


## P1 == I || P2 == I
If either point is the point at infinity, we return the other point.

## x1 ≠ x2
When we have points where the x’s differ, we can add using a fairly simple formula. 
- We’ll first find the slope created by the two points:
    - `s = (y2 - y1)/(x2 - x1)`  
- We can use the slope to calculate x3. Once we know x3, we can
calculate y3. 
    - `x3 = s2 - x1 - x2`
    - `y3 = s(x1 - x3) - y1`

## x1 == x2 && y1 != y2
When the two points are additive inverses (that is, they have the same x but a different y, causing a vertical line). This should return the point at infinity.

## P1 = P2
We have to calculate the line that’s tangent to the curve at P1 and find the point at which the line intersects the curve. 
- We’ll find the slope of the tangent point: 
    - s = (3x1^2 + a)/(2y1)
- The rest of the formula:
    - `x3 = s^2 - 2x1`
    - `y3 = s(x1 - x3) - y1`

## One More Exception
This can only happen if P1 = P2 and the y coordinate is 0, in which case the slope calculation will end up with a 0 in the denominator. 
If the two points are equal and the y coordinate is 0, we return the point at infinity.