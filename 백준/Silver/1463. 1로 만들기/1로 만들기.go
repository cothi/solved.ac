package main

import "fmt"

func min(a int, b int) int {
	if a < b {
		return a
	} else {
		return b
	}
}

func main() {
	var n int
	fmt.Scanf("%d", &n)
	vec := make([]int, n+1)
	vec[0] = 0
	vec[1] = 0

	for i := 2; i <= n; i++ {
		vec[i] = vec[i-1] + 1

		if i%2 == 0 {
			vec[i] = min(vec[i/2]+1, vec[i])
		}
		if i%3 == 0 {
			vec[i] = min(vec[i/3]+1, vec[i])
		}

	}

	fmt.Println(vec[n])
}