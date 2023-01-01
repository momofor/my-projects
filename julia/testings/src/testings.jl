using LinearAlgebra
mk_horizontal(vector) = reshape(vector, 2, 1)

inputs = [3 4 15
    12 -5 -3]

vals = mk_horizontal([inputs[1, 3], inputs[2, 3]])
x_vals = mk_horizontal([inputs[1, 1], inputs[2, 1]])
y_vals = mk_horizontal([inputs[1, 2], inputs[2, 2]])

Determinant = det([x_vals y_vals])
Determinant_x = det([vals y_vals])
Determinant_y = det([x_vals vals])

x = Determinant_x / Determinant
y = Determinant_y / Determinant

println("value of x:$x; value of y:$y ")
