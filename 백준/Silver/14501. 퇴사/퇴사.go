package main

import "fmt"

var (
	maxNum  int
	maxSize int
)

func top_down(t [1000]int, p [1000]int, n int, profit int) {
	if n > maxSize {
		return
	}

	if n == maxSize {
		if maxNum < profit {
			maxNum = profit
		}
		return
	}
	top_down(t, p, n+t[n], profit+p[n])
	top_down(t, p, n+1, profit)
}

func main() {
	var a, b int
	fmt.Scanf("%d", &maxSize)

	var t, p [1000]int
	for i := 0; i < maxSize; i++ {
		fmt.Scanf("%d %d", &a, &b)
		t[i] = a
		p[i] = b
	}

	top_down(t, p, 0, 0)
	fmt.Println(maxNum)
}