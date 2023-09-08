package kata_test

import (
	"testing"

	. "codewars.dzfl.pl/kata"
)

func TestFoo(t *testing.T) {
	examples := [...][2]int{
		{1, 1},
		{8, 36},
		{22, 253},
		{100, 5050},
		{213, 22791},
	}
	for i := 0; i < len(examples); i++ {
		v := examples[i]
		result := Summation(v[0])
		if result != v[1] {
			t.Errorf("Failed. Expected %d, got %d", v[1], result)
		}
	}
}
