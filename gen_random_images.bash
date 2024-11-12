mkdir test_images
cd test_images

for i in {1..100}
do
    # Generate a random string of 10 a-zA-Z characters
    SEED=$(cat /dev/urandom | tr -dc 'a-zA-Z' | fold -w 10 | head -n 1)
    
    # Run the wget command with the random seed
    wget "https://picsum.photos/seed/$SEED/200/300" -O "image_$i.jpg"
done
