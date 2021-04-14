class Plugboard:
    def __init__(self, couples):
        self.list = couples
        if len(self.list) - 1 < 10:
            self.dict = {}
            self.dictify()
        else:
            raise Exception("too many couples of letters ("+str(len(self.list))+")")
        
    def dictify(self):
        for c in self.list:
            if not self.dict.get(c[0]):
                self.dict[c[0]] = c[1]
                self.dict[c[1]] = c[0]
            else:
                self.dict = {}
                raise Exception("Duplicated letter", c[0])
    
    def apply(self, text):
        new_text = []
        for c in text:
            if self.dict.get(c):
                new_text.append(self.dict[c])
            else:
                new_text.append(c)
        return "".join(new_text)
        
