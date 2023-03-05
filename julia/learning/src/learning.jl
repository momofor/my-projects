using Symbolics
using Latexify
function test()
    @variables $α::Real
end

z = 4α + α

print(z)
latexify(z)
