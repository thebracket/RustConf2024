# The Challenge: Data Sets

The 1BRC provides a big list of weather stations:

```csv
# Adapted from https://simplemaps.com/data/world-cities
# Licensed under Creative Commons Attribution 4.0 (https://creativecommons.org/licenses/by/4.0/)
Tokyo;35.6897
Jakarta;-6.1750
Delhi;28.6100
Guangzhou;23.1300
Mumbai;19.0761
Manila;14.5958
Shanghai;31.1667
```

There's 43,000 of them - so we're not going to list them here. (They are in `code/data_collector/weather_stations.csv`
if you'd like to read it.)

The *data creator* produces a file with 1,000,000,000 rows. Each is a randomly picked weather station from the list,
and creates a random temperature based on the second column - the average temperature. It uses a Gaussian distribution
to give a normal-looking range of temperatures.

The readings look like this:

```csv
Shuanghe;40.1
Motomachi;37.7
Cedros;21.4
Antsenavolo;-18.1
Huaiyang;26.7
Muntinlupa City;16.1
São José do Belmonte;-1.4
Curacaví;-37.7
Santa Ana;25.8
New Castle;36.9
Lawrenceville;16.5
Narsarsuaq;57.3
Hongsi;38.6
```

> Obviously, I'm not going to fill the projector with a billion rows.