class Box {
    int width
    int height

    // opinion: I prefer something like
    // fun void Box
    // or just
    // void Box
    // I don't like the colon-style type stating
    fun Box(int width, int height) : void {
        this.width = width
        this.height = height
    }
}