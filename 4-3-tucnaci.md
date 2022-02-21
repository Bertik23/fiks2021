# Řešení 5. úlohy 2. kola - Prohlídkové okruhy

## Problémy ukázkového protokolu
Kromě zmíněné replay attack má ukázkový protokol i jiné problémy.
* Odeslaná zpráva je stejně dlouhá jako zdrojová zpráva, a protože má každá možná zpráva jinou délku, tak se dá jedoduše určit, podle počtu znaků, o jakou zprávu se jedná.
* Stejným způsobem můžeme odhadnou argument zprávy `POSLETE MI`, podle délky můžeme odhadnou řádovou velikost.
* Pokud bude nějaká zpráva zadržena, tak se druhá strana nedozví o tom, že se zpráva někam ztratila.

## Návrh protokolu
Zpráva odeslaná se skládá z několika částí $Z_1.Z_2.Z_3.Z.4$

* $Z_1$ je aktuální čas v mikrosekundách
* $Z_2$ je vlastní obsah zprávy