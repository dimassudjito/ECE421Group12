package group12.core;

import java.util.function.Predicate;
import java.util.stream.Stream;
import java.util.Optional;

public class PickShareFunctional {
    public static Optional<ShareInfo> findHighPriced(Stream<String> stream) {
		final Predicate isPriceLessThan500 = ShareUtil.isPriceLessThan(500);
		try {
			return Optional.of(stream
		            .map(symbol -> ShareUtil.getPrice(symbol))
		            .filter(shareInfo -> isPriceLessThan500.test(shareInfo))
		            .max((x, y) -> x.price.compareTo(y.price)).get());
		} catch (Exception e) {
			return Optional.empty();
		}
    }

	public static void main(String[] args) {
		long startTime = System.nanoTime();
		Optional<ShareInfo> highPriced = findHighPriced(Shares.symbols.stream());
		long endTime = System.nanoTime();
		System.out.println("Time taken: " + (endTime-startTime)/1000000 + "ms");
		
		if (highPriced.isPresent()) {
			System.out.println("High priced under $500 is " + highPriced.get());
		} else {
			System.out.println("Value is currently not available");
		}
	}
}
