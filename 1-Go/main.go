package main

import "fmt"

// Go has no enum keyword: the idiom is a named int type plus iota (Pending == 0).
type Status int

const (
	Pending Status = iota
	Running
	Succeeded
	Failed
)

func (s Status) String() string {
	return [...]string{"pending", "running", "succeeded", "failed"}[s]
}

// No tagged unions either: a sealed interface implemented by a fixed set of structs.
type Media interface{ isMedia() }

type Text struct{ Length int }
type Image struct{ Width, Height int }
type Video struct {
	Width, Height int
	Duration      float64
}

func (Text) isMedia()  {}
func (Image) isMedia() {}
func (Video) isMedia() {}

// A type switch is Go's pattern match over the variants.
func describe(m Media) string {
	switch v := m.(type) {
	case Text:
		return fmt.Sprintf("text: %d chars", v.Length)
	case Image:
		return fmt.Sprintf("image: %dx%d", v.Width, v.Height)
	case Video:
		return fmt.Sprintf("video: %dx%d, %.1fs", v.Width, v.Height, v.Duration)
	}
	return "unknown"
}

func main() {
	s := Running
	fmt.Printf("status: %s (%d)\n", s, s)
	for _, m := range []Media{
		Text{280}, Image{1920, 1080}, Video{1920, 1080, 12.5},
	} {
		fmt.Println(describe(m))
	}
}
