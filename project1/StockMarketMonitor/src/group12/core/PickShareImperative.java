package group12.core;

import java.util.function.Predicate;

/**
 * The original code snippet doesn't compile and it's unclear where it should be located.
 * Current changes: create highPriced field and initiate it with shareInfo.
 */
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
