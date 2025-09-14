package main

import (
	"fmt"
	"image"
	"os"

	"github.com/hajimehoshi/guigui"
	"github.com/hajimehoshi/guigui/basicwidget"
)

type Root struct {
	guigui.DefaultWidget

	background basicwidget.Background
}

func main() {
	op:= &guigui.RunOptions{
		Title: "Bygonization",
		WindowSize: image.Pt(800, 600),
	}
	if err:= guigui.Run(&Root{}, op); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
