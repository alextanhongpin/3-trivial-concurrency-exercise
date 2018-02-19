import concurrent.futures
import urllib.request

URLS = ['http://www.foxnews.com/',
        'http://www.cnn.com/',
        'http://europe.wsj.com/',
        'http://www.bbc.co.uk/',
        'http://some-made-up-domain.com/']

def load_url(url):
  with urllib.request.urlopen(url, timeout = 60) as conn:
    return conn.read()

with concurrent.futures.ThreadPoolExecutor(max_workers = 5) as executor:
  future_to_url = {executor.submit(load_url, url): url for url in URLS}
  for future in concurrent.futures.as_completed(future_to_url):
    url = future_to_url[future]
    try:
      data = future.result()
    except Exception as exc:
      print('{} generated as exception: {}'.format(url, exc))
    else:
      print('{} page is {} bytes'.format(url, len(data)))