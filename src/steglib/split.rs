pub trait Split {
    /**
     * Given the data of a file as Vec<u8>, returns Vec<Vec<u8>> which is that same file
     * redistributed among "buckets". Each index of the return is a 1:1 correspondence to the order
     * of the buckets given in `bucket_capacities`. 
     */ 
    fn split(data: Vec<u8>, bucket_capacities: Vec<u64>) -> Vec<Vec<u8>>;

    /**
     * Given the exact return output of `split` (buckets in right index), returns the Vec<u8> that
     * was used to create it.
     */ 
    fn join(data: Vec<Vec<u8>>) -> Vec<u8>;
}


/**
 * Scrambles a file into pieces. For example:
 *
 * Input file: this is a text
 * 
 * If there are three buckets:
 *
 * #1: tss s
 * #2: h  tt
 * #3: iiae
 *
 * A limitation of this approach is that each bucket has the same size. This means that if the
 * sizes of the buckets are different, at most only min(bucket_sizes) * buckets.len() can be written to.
 */
pub struct SplitScrambled;


impl Split for SplitScrambled {
    fn split(data: Vec<u8>, bucket_capacities: Vec<u64>) -> Vec<Vec<u8>> {

        let mut scrambled_content: Vec<Vec<u8>> = vec![Vec::new(); bucket_capacities.len()];


        // Scramble data into buckets
        let mut next_bin: usize = 0;
        for number in data {
            scrambled_content[next_bin].push(number);
            next_bin = (next_bin + 1) % bucket_capacities.len();
        }


        scrambled_content
    }

    fn join(data: Vec<Vec<u8>>) -> Vec<u8> {
        let mut total_size: usize = 0;
        for piece in &data {
            total_size += piece.len();
        }

        let mut unified_piece: Vec<u8> = vec![0; total_size];
        let mut offset: usize = 0;
        let bucket_count = data.len();


        for piece in data {
            let mut piece_num: usize = 0;
            for byte in piece {
                unified_piece[offset + piece_num * bucket_count] = byte;
                piece_num += 1;
            }

            offset += 1;
        }

        unified_piece
    }
}
