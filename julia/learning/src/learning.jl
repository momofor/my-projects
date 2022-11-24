# Using the derivation rule: d(x^n) = n*x^-1

function derivative_of_thing(x, power)
    return power * x^power - 1
end

input = readline()
captures = match(r"^(\d)\^([0-9])$", input).captures
x = captures[1]
y = parse(Int64, captures[2])
y_1 = y - 1

println("x=", x, ";y=", y)
println("d(", x, "^", y, ") = ", y, "*", x, "^", y - 1)

println(derivative_of_thing(5, 2))
