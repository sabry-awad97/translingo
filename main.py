# %%
import pandas as pd
import googletrans
from googletrans import Translator


translator = Translator()

pd.set_option('max_colwidth', 300)

# how to get the supported language and their corresponing code
lang_df = pd.DataFrame.from_dict(
    googletrans.LANGUAGES,  orient='index', columns=['Language'])
lang_df


text = "Cerco un centro di gravità permanente"


my_example = translator.detect(text)


my_translation = translator.translate(text, dest='en', src='auto')


print(my_translation.text)


my_example = pd.DataFrame({'EnglishText': ['Police in France say they have seized 140,000 face masks that were destined for sale on the black market.',
                                           'Officers say they discovered the haul when they spotted a businessman unloading the masks from a lorry into a house in St Denis, north of Paris.',
                                           'Prime Minister Boris Johnson could possibly lead the daily news conference on Monday but, if not then, it will be pretty soon afterwards.']})


my_example['ItalianText'] = my_example['EnglishText'].apply(
    lambda x: translator.translate(x, src='en', dest='it').text)
my_example
