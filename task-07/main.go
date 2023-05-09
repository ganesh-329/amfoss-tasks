package main

import (
	"strconv"
	"syscall/js"
)

type Response struct {
	CurrentValue int64  `json:"current_value"`
	Error        string `json:"error,omitempty"`
}

func increment(this js.Value, args []js.Value) interface{} {
	elem := js.Global().Get("document").Call("getElementById", "number")

	if elem.Get("textContent").String() != "Go!" {
		number, err := strconv.ParseInt(elem.Get("textContent").String(), 10, 64)
		if err != nil {
			return Response{Error: err.Error()}
		}
		number += int64(args[0].Int())
		elem.Set("textContent", number)
		return Response{CurrentValue: number}
	} else {
		elem.Set("textContent", 1)
		return Response{CurrentValue: 1}
	}
}

func decrement(this js.Value, args []js.Value) interface{} {
	elem := js.Global().Get("document").Call("getElementById", "number")

	if elem.Get("textContent").String() != "Go!" {
		number, err := strconv.ParseInt(elem.Get("textContent").String(), 10, 64)
		if err != nil {
			return Response{Error: err.Error()}
		}
		number -= int64(args[0].Int())
		elem.Set("textContent", number)
		return Response{CurrentValue: number}
	} else {
		elem.Set("textContent", -1)
		return Response{CurrentValue: -1}
	}
}

func reset(this js.Value, args []js.Value) interface{} {
	elem := js.Global().Get("document").Call("getElementById", "number")
	elem.Set("textContent", 0)
	return Response{CurrentValue: 0}
}

func main() {
	js.Global().Set("goIncrement", js.FuncOf(increment))
	js.Global().Set("goDecrement", js.FuncOf(decrement))
	js.Global().Set("goReset", js.FuncOf(reset))
	select {}
}
