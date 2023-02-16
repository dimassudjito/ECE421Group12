package group12.core;

import java.io.BufferedReader;

import java.io.IOException;
import java.io.InputStreamReader;
import java.math.BigDecimal;
import java.net.URL;
import java.net.URLConnection;
import java.util.concurrent.ThreadLocalRandom;

public class APIFinance {
	private static final String BASE_URL = "https://www.alphavantage.co/query?";
	private final static String[] apiKeys = new String[]{"S1QYMYBVKO6BFAKK","4WOAUYT5Z6NTQ4U4"};
	private static int apiKeyNum = 0;
	private final static String tooManyCallsResponse = "Thank you for using Alpha Vantage! Our standard API call frequency is 5 calls per minute and 500 calls per day. Please visit https://www.alphavantage.co/premium/ if you would like to target a higher API call frequency.";
	
	public static BigDecimal getPrice(final String symbol) {
		boolean success = false;
		boolean retryRequest = false;
		BigDecimal price = new BigDecimal(-1);
		while (true) {
			if (success) {
				break;
			}
			try {
				retryRequest = false;
				URL url = null;
				url = new URL(BASE_URL + "function=GLOBAL_QUOTE&symbol=" + symbol + "&apikey=" + apiKeys[apiKeyNum]);
				apiKeyNum = (apiKeyNum + 1) % apiKeys.length;
				
				URLConnection connection = url.openConnection();
				InputStreamReader inputStream = new InputStreamReader(connection.getInputStream(), "UTF-8");
				BufferedReader bufferedReader = new BufferedReader(inputStream);
				String line;
				
				while ((line = bufferedReader.readLine()) != null) {
					if (line.contains("price")) {
						price = new BigDecimal(line.split("\"")[3].trim());
						// Successful! Can leave loop now :)
						success = true;
					} else if (line.contains(tooManyCallsResponse)) {
						bufferedReader.close();
						retryRequest = true;
						break;
					}
				}
				if (retryRequest) {
					// The API complained we're spamming it with too many requests!
					Thread.sleep(ThreadLocalRandom.current().nextInt(1000, 5000));
					// We must loop again and remake this request
					bufferedReader.close();
					continue;
				}
				bufferedReader.close();
				if (!success) {
					// This happened because the response didn't include a price field. This could be caused by requesting a stock ticker that doesn't exist *cough* FB
				}
			} catch (IOException e) {
				System.out.println("failure sending request");
				continue;
			} catch (Exception e) {
				System.out.println("Random exception!");
				continue;
			}
		}
		return price;
	}
}
