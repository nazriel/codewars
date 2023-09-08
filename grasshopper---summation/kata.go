package kata

func Summation(n int) int {
	accum := 0
	for i := 0; i <= n; i++ {
		accum += i
	}
	return accum
}
