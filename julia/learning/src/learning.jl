input = readline()
regexu = r"(\d) (\d) (\d) (\d) (\d) (\d)"
captures = match(regexu, input)

(a_1, a_2, k_1) = (parse(Int,captures[1]), parse(Int,captures[2]), parse(Int,captures[3]))
(b_1, b_2, k_2) = (parse(Int,captures[4]), parse(Int,captures[5]), parse(Int,captures[6]))

c = [a_1 a_2 k_1
    b_1 b_2 k_2]
println(c)
