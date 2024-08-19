use std::cmp::min;

#[allow(dead_code)]
pub trait Split {
    /**
     * Split Vec<u8> into Vec<Vec<u8>>, where each vec is filled to less than to equal to the
     * corresponding size in `bin_capacities`. This does not modify `data`. Any remaining data that
     * is not filled will be set to 0.
     */ 
    fn split_to_bins(data: &Vec<u8>, bin_capacities: &Vec<u64>) -> Vec<Vec<u8>>;

    /**
     * Undo split_to_bins. Does not modify `data`.
     */ 
    fn join_bins(data: &[Vec<u8>]) -> Vec<u8>;
}


/**
 * Fill any remaining space of each element of `bins` with 0 so that its length corresponds to the
 * element in `bin_capacities`.
 */ 
fn inflate_bins(bins: &mut Vec<Vec<u8>>, bin_capacities: &Vec<u64>) {
    while bins.len() < bin_capacities.len() {
        bins.push(Vec::new());
    }

    let mut index = 0;
    while index < bin_capacities.len() {
        let remaining_elements = (bin_capacities[index] as usize) - bins[index].len();

        if remaining_elements > 0 {
            bins[index].extend(vec![0; remaining_elements]);
        }

        index += 1;
    }
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
    fn split_to_bins(data: &Vec<u8>, bin_capacities: &Vec<u64>) -> Vec<Vec<u8>> {
        let cloned_data = data.clone();
        let mut scrambled_content: Vec<Vec<u8>> = vec![Vec::new(); bin_capacities.len()];

        // Scramble data into buckets
        let mut next_bin: usize = 0;
        for number in cloned_data {
            scrambled_content[next_bin].push(number);
            next_bin = (next_bin + 1) % bin_capacities.len();
        }
        
        inflate_bins(&mut scrambled_content, &bin_capacities);


        scrambled_content
    }


    fn join_bins(data: &[Vec<u8>]) -> Vec<u8> {
        let total_byte_count: usize = data.iter().map(|v| v.len()).sum();
        let mut unified_piece: Vec<u8> = Vec::with_capacity(total_byte_count);
        let mut offset: usize = 0;
        let bucket_count = data.len();

        for piece in data {
            let mut piece_num: usize = 0;
            for byte in piece.clone() {
                unified_piece[offset + piece_num * bucket_count] = byte;
                piece_num += 1;
            }

            offset += 1;
        }

        unified_piece
    }
}







/**
 * Maximizes available file space by splitting file into chunks.
 *
 * Input file: this is a text
 * 
 * If there are three bins:
 *
 * #1 (3 bytes): thi
 * #2 (6 bytes): s is a
 * #3 (10000 bytes): text
 *
 * Buckets are filled in the order they are passed in. 
 */
pub struct SplitChunks;


impl Split for SplitChunks {
    fn split_to_bins(data: &Vec<u8>, bin_capacities: &Vec<u64>) -> Vec<Vec<u8>> {
        let mut cloned_data = data.clone();
        let mut bins = Vec::with_capacity(bin_capacities.len());
        let mut index = 0;

        while cloned_data.len() > 0 {
            // Capacity of the bin to fill
            let capacity = bin_capacities[index];
            index += 1;

            // Read at most that many bytes to fill the bin.
            let bytes_to_drain: usize = min(capacity as usize, cloned_data.len());
            let buffer: Vec<u8> = cloned_data.drain(..bytes_to_drain).collect();

            bins.push(buffer);
        }

        inflate_bins(&mut bins, &bin_capacities);
        bins
    }

    fn join_bins(data: &[Vec<u8>]) -> Vec<u8> {
        let total_byte_count: usize = data.iter().map(|v| v.len()).sum();
        let mut unified_piece: Vec<u8> = Vec::with_capacity(total_byte_count);

        for piece in data {
            unified_piece.extend(piece.clone());
        }

        unified_piece
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_full() {
        let data: Vec<u8> = vec!(10, 20);
        let buckets_1: Vec<u64> = vec!(3, 5);
        let buckets_2: Vec<u64> = vec!(1, 5);
        let buckets_3: Vec<u64> = vec!(1, 5, 1, 1, 1, 3);
        let buckets_4: Vec<u64> = vec!(2);

        // Split the data, test for not modifying variables.
        assert_eq!(SplitChunks::split_to_bins(&data, &buckets_1), vec!(vec!(10, 20, 0), vec!(0, 0, 0, 0, 0)));
        assert_eq!(SplitChunks::split_to_bins(&data, &buckets_2), vec!(vec!(10), vec!(20, 0, 0, 0, 0)));
        assert_eq!(SplitChunks::split_to_bins(&data, &buckets_3), vec!(vec!(10), vec!(20, 0, 0, 0, 0), vec!(0), vec!(0), vec!(0), vec!(0,0,0)));
        assert_eq!(SplitChunks::split_to_bins(&data, &buckets_4), vec!(vec!(10, 20)));

        assert_eq!(data, vec!(10, 20));
    }

    #[test]
    fn test_split_scrambled() {
        let data: Vec<u8> = vec!(10, 20, 30, 40, 50);
        let buckets_1: Vec<u64> = vec!(3, 5);
        let buckets_2: Vec<u64> = vec!(3, 3, 3);
        let buckets_3: Vec<u64> = vec!(5);

        // Split the data, test for not modifying variables.
        assert_eq!(SplitScrambled::split_to_bins(&data, &buckets_1), vec!(vec!(10, 30, 50), vec!(20, 40, 0, 0, 0)));
        assert_eq!(SplitScrambled::split_to_bins(&data, &buckets_2), vec!(vec!(10, 40, 0), vec!(20, 50, 0), vec!(30, 0, 0)));
        assert_eq!(SplitScrambled::split_to_bins(&data, &buckets_3), vec!(vec!(10, 20, 30, 40, 50)));

        assert_eq!(data, vec!(10, 20, 30, 40, 50));
    }

}




