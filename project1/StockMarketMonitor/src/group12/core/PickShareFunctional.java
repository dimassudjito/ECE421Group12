package group12.core;

import java.util.function.Predicate;
import java.util.stream.Stream;


public class PickShareFunctional {
    public static ShareInfo findHighPriced(Stream<String> stream) {
		final Predicate isPriceLessThan500 = ShareUtil.isPriceLessThan(500);
        return stream
            .map(symbol -> ShareUtil.getPrice(symbol))
            .filter(shareInfo -> isPriceLessThan500.test(shareInfo))
            .max((x, y) -> x.price.compareTo(y.price)).get();
    }

	public static void main(String[] args) {
		ShareInfo highPriced = findHighPriced(Shares.symbols.stream());
		System.out.println("High priced under $500 is " + highPriced);
	}
}
