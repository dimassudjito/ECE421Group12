package group12.core;

import java.util.function.Predicate;

public class PickShareImperative {
	private static ShareInfo highPriced;

	public static void main(String[] args) {
		final Predicate isPriceLessThan500 = ShareUtil.isPriceLessThan(500);
			for (String symbol : Shares.symbols) {
				ShareInfo shareInfo = ShareUtil.getPrice(symbol);
				if (isPriceLessThan500.test(shareInfo))
					if (highPriced == null) highPriced = shareInfo;
					highPriced = ShareUtil.pickHigh(highPriced, shareInfo);
			}
			System.out.println("High priced under $500 is " + highPriced);
	}
}
