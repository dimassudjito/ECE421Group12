package group12.core;

import java.util.Collection;
import java.util.Comparator;
import java.util.function.Predicate;
import java.util.stream.Collectors;
import java.util.stream.Stream;


public class PickShareFunctional {
    private static ShareInfo highPriced;

    public static ShareInfo findHighPriced(Stream<String> stream) {
		final Predicate isPriceLessThan500 = ShareUtil.isPriceLessThan(500);
        return stream
            .map(ShareUtil::getPrice)
            .filter(isPriceLessThan500::test)
            .max((x, y) -> x.price.compareTo(y.price)).get();
    }

	public static void main(String[] args) {
		final Predicate isPriceLessThan500 = ShareUtil.isPriceLessThan(500);
		for (String symbol : Shares.symbols) {
			ShareInfo shareInfo = ShareUtil.getPrice(symbol);
			if (isPriceLessThan500.test(shareInfo))
				if (highPriced == null) highPriced = shareInfo;
				highPriced = ShareUtil.pickHigh(highPriced, shareInfo);
		}
		System.out.println("High priced under $500 is " + highPriced);
		System.out.println("Test test test test test test test test");


        highPriced = findHighPriced(Shares.symbols.stream());
		System.out.println("High priced under $500 is " + highPriced);

	}
}
