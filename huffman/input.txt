{-# LANGUAGE GADTSyntax #-}

import Data.Binary.Get qualified as G
import Data.Binary.Put qualified as P
import Data.Bits
import Data.ByteString.Internal qualified as I
import Data.ByteString.Lazy qualified as L
import Data.List
import Data.Word

type Reg = (Word8, Word32)

data Huffman where
  Folha :: Int -> Char -> Huffman
  No :: Int -> Huffman -> Huffman -> Huffman
  deriving (Show)

instance Eq Huffman where
  (==) :: Huffman -> Huffman -> Bool
  a == b = pegaNumero a == pegaNumero b

instance Ord Huffman where
  (<=) :: Huffman -> Huffman -> Bool
  a <= b = pegaNumero a <= pegaNumero b

ordena :: (Ord a) => [a] -> [a]
ordena [] = []
ordena [x] = [x]
ordena l@(x : xs) = ordena (filter (< x) xs) ++ filter (== x) l ++ ordena (filter (> x) xs)


count :: (Eq a) => a -> [a] -> Int
count x = length . filter (x ==)

freqSimb :: String -> [Huffman]
freqSimb s = ordena $ go s
  where
    go [] = []
    go s@(x : xs) = Folha (count x s) x : go (filter (x /=) xs)

pegaNumero :: Huffman -> Int
pegaNumero (Folha i c) = i
pegaNumero (No i l r) = i

construirArvore :: [Huffman] -> Huffman
construirArvore [] = error "Arvore Vazia"
construirArvore [x] = x
construirArvore (x : y : xs) = construirArvore $ insere (No (pegaNumero x + pegaNumero y) x y) xs
  where
    insere a [] = [a]
    insere a l@(x : xs)
      | a <= x = a : l
      | otherwise = x : insere a xs

codHuffman :: Huffman -> [(Char, String)]
codHuffman = go ""
  where
    go s (No i l r) = go (s ++ "0") l ++ go (s ++ "1") r
    go s (Folha i c) = [(c, s)]

codificar :: String -> Huffman -> String
codificar "" _ = ""
codificar s@(x : xs) h = concat $ go s $ codHuffman h
  where
    go [] _ = []
    go s@(x : xs) huffmanCode = map snd (filter ((x ==) . fst) huffmanCode) ++ go xs huffmanCode

decodificar :: String -> Huffman -> String
decodificar [] _ = []
decodificar s@(x : xs) h@(No i l r) =  let (caractere, resto) = go s (No i l r) in caractere : decodificar resto h
  where 
    go s (Folha i c) = (c, s)
    go s@(x : xs) h@(No i l r)
      | x == '0' = go xs l
      | x == '1' = go xs r

freq :: (Eq a) => [a] -> [(a, Int)]
freq [] = []
freq (x : xs) = let (l1, l2) = partition (== x) xs in (x, length l1 + 1) : freq l2

putStart :: (Int, Int) -> P.Put
putStart (n, t) =
  do
    P.putWord8 $ toEnum n
    P.putWord32be $ toEnum t
    P.flush

putFreqList :: [(Char, Int)] -> P.Put
putFreqList [] = P.flush
putFreqList ((c, f) : xs) =
  do
    P.putWord8 $ I.c2w c
    P.putWord32be $ toEnum f
    putFreqList xs

s2w :: String -> Word8
s2w string = go string 7 where
    go []     _ = 0
    go (x:xs) n = shift (read [x]) n + go xs (n - 1)

w2s :: Word8 -> String
w2s word = go word 128 where
    go :: Word8 -> Word8 -> String
    go _ 0 = []
    go w n | w .&. n == 0 = '0' : go w (shift n $ -1)
           | otherwise    = '1' : go w (shift n $ -1)

putFullTxt :: String -> P.Put
putFullTxt [] = P.flush
putFullTxt s =
  do
    P.putWord8 $ s2w $ take 8 s
    putFullTxt $ drop 8 s

getReg :: G.Get Reg
getReg =
  do
    c <- G.getWord8
    f <- G.getWord32be
    return (c, f)

getRegs :: G.Get [Reg] -- argumento da funcao propagado pelo pipeline da monada
getRegs =
  do
    empty <- G.isEmpty
    if empty
      then return []
      else do r <- getReg; rs <- getRegs; return (r : rs)

printRegs :: [Reg] -> IO ()
printRegs [] = return ()
printRegs ((c, f) : rs) =
  do
    putStrLn $ (show $ I.w2c c) ++ " - " ++ show f
    printRegs rs

printStart :: Reg -> IO ()
printStart (n, t) =
  do
    putStrLn $ show n ++ " - " ++ show t

getC :: G.Get Word8
getC = G.getWord8 >>= \x -> return x

getMsg :: G.Get [Word8] -- argumento da funcao propagado pelo pipeline da monada
getMsg =
  do
    empty <- G.isEmpty
    if empty
      then return []
      else do x <- getC; xs <- getMsg; return (x : xs)

printMsg :: [Word8] -> IO ()
printMsg [] = return ()
printMsg (x : xs) = (putStrLn $ show x) >> printMsg xs


reg2LeafList :: [Reg] -> [Huffman]
reg2LeafList r = ordena $ go r
  where
    go [] = []
    go ((c, f) : rs) = (Folha (read $ show f) $ I.w2c c) : go rs

escrita :: IO ()
escrita =
  do
    txt <- readFile "file.txt"
    let xs = freq txt
    let fr = freqSimb txt
    let tr = construirArvore fr
    let hf = codHuffman tr
    let final = codificar txt tr
    let uniqueCharacters = length xs
    let totalCharacters = 8 - rem (length final) 8
    let bs1 = P.runPut $ putStart (uniqueCharacters, totalCharacters)
    let bs = P.runPut $ putFreqList xs
    let bs2 = P.runPut $ putFullTxt final
    L.writeFile "file.bin" $ bs1 <> bs <> bs2

leitura :: IO ()
leitura =
  do
    bs <- L.readFile "file.bin"
    let regHead@(n, t) = G.runGet getReg bs
    let regTail = G.runGet getRegs $ L.take ((read $ show n) * 5) $ L.drop 5 bs
    let msg = G.runGet getMsg $ L.drop ((read $ show n) * 5 + 5) bs
    printStart regHead
    printRegs regTail
    let fr = reg2LeafList regTail
    let binMsg' = concat $ map w2s msg
    let binMsg = take (length binMsg' - fromEnum t) binMsg'
    let msgDecodificada = decodificar binMsg $ construirArvore fr
    -- print t
    -- print $ length binMsg' - fromEnum t
    writeFile "out.txt" msgDecodificada

passoAPasso :: IO () =
  do
    putStrLn "Digite uma String"
    ln <- getLine
    let fr = freqSimb ln
    print fr
    putStrLn ""
    let tr = construirArvore fr
    print tr
    putStrLn ""
    let hf = codHuffman tr
    print hf
    putStrLn ""
    let final = codificar ln tr
    print $ "codificado: " ++ final
    print $ getBytes final
    putStrLn ""
    putStrLn ""
    print $ "decodificado: " ++ decodificar final tr

getBytes :: String -> [Word8]
getBytes "" = []
getBytes s = (s2w $ take 8 s) : (getBytes $ drop 8 s)


main :: IO ()
main = 
  do
    escrita
    leitura