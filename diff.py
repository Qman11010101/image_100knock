from PIL import Image

def compare_images(image1_path, image2_path):
    image1 = Image.open(image1_path).convert("RGB")
    image2 = Image.open(image2_path).convert("RGB")

    if image1.size != image2.size:
        raise ValueError("Images must be the same size")

    width, height = image1.size

    for y in range(height):
        for x in range(width):
            pixel1 = image1.getpixel((x,y))
            pixel2 = image2.getpixel((x,y))

            if pixel1 != pixel2:
                print(f"Pixel at ({x}, {y}): {pixel1}, {pixel2}")

image1_path = "./input.png"
image2_path = "./q9_gaussian_filter.png"
compare_images(image1_path, image2_path)