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
		int callNum = ThreadLocalRandom.current().nextInt(0, 1000);
		BigDecimal price = new BigDecimal(-1);
		System.out.println("Call num: " + callNum);
		while (true) {
			if (success) {
				break;
			}
			try {
				Thread.sleep(ThreadLocalRandom.current().nextInt(1000, 5000));
				retryRequest = false;
				URL url = null;
				System.out.println("Using API key " + apiKeyNum + " for call " + callNum + " and the url is " + BASE_URL + "function=GLOBAL_QUOTE&symbol=" + symbol + "&apikey=" + apiKeys[apiKeyNum]);
				url = new URL(BASE_URL + "function=GLOBAL_QUOTE&symbol=" + symbol + "&apikey=" + apiKeys[apiKeyNum]);
				apiKeyNum = (apiKeyNum + 1) % apiKeys.length;
				
				URLConnection connection = url.openConnection();
				InputStreamReader inputStream = new InputStreamReader(connection.getInputStream(), "UTF-8");
				BufferedReader bufferedReader = new BufferedReader(inputStream);
				String line;
				
				while ((line = bufferedReader.readLine()) != null) {
					if (line.contains("price")) {
						price = new BigDecimal(line.split("\"")[3].trim());
						System.out.println("Successful! Can leave loop now :) for call " + callNum);
						success = true;
					} else if (line.contains(tooManyCallsResponse)) {
						bufferedReader.close();
						retryRequest = true;
						break;
					}
				}
				if (retryRequest) {
					System.out.println("The API complained we're spamming it with too many requests! Call " + callNum);
					Thread.sleep(ThreadLocalRandom.current().nextInt(2000, 8000));
					// We must loop again and remake this request
					bufferedReader.close();
					continue;
				}
				bufferedReader.close();
				if (!success) {
					System.out.println("Call " + callNum + " Why the heck did it escape the loop?????");
				}
			} catch (IOException e) {
				System.out.println("failure sending request for call " + callNum);
				continue;
			//} catch (InterruptedException e) {
			//	Thread.currentThread().interrupt();
			} catch (Exception e) {
				System.out.println("Random exception! in call " + callNum);
				continue;
			}
		}
		System.out.println("The price for " + symbol + " is " + price + " for call " + callNum);
		return price;
	}
}
