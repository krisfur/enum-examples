#include <stdio.h>

// A plain enum: just named integer constants (PENDING == 0, etc.).
enum Status { PENDING, RUNNING, SUCCEEDED, FAILED };

// A "tagged union": the `kind` field says which union member is valid.
// C does NOT enforce this — reading the wrong member is undefined behaviour.
struct Media {
    enum { TEXT, IMAGE, VIDEO } kind;
    union {
        struct { double length; } text;
        struct { double width, height; } image;
        struct { double width, height, duration; } video;
    };
};

void describe(struct Media m) {
    switch (m.kind) {
        case TEXT:  printf("text: %.0f chars\n", m.text.length); break;
        case IMAGE: printf("image: %.0fx%.0f\n", m.image.width, m.image.height); break;
        case VIDEO: printf("video: %.0fx%.0f, %.1fs\n",
                        m.video.width, m.video.height, m.video.duration); break;
    }
}

int main(void) {
    const char *names[] = {"pending", "running", "succeeded", "failed"};
    enum Status s = RUNNING;
    printf("status: %s (%d)\n", names[s], s);

    struct Media items[] = {
        { .kind = TEXT,  .text  = { .length = 280 } },
        { .kind = IMAGE, .image = { .width = 1920, .height = 1080 } },
        { .kind = VIDEO, .video = { .width = 1920, .height = 1080, .duration = 12.5 } },
    };
    for (size_t i = 0; i < sizeof items / sizeof items[0]; i++)
        describe(items[i]);
    return 0;
}
